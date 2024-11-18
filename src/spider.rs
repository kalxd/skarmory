use crate::state::AppState;
use crate::{error::Result, state::write_state};
use futures::{stream, StreamExt, TryStreamExt};
use scraper::{ElementRef, Html, Selector};

const JANDAN_URL: &str = "https://jandan.net/pic";

fn el_href(el: ElementRef<'_>) -> Option<String> {
	el.attr("href").map(|x| format!("https:{x}"))
}

#[derive(Debug)]
struct JandanPage {
	img_link_list: Vec<String>,
	next_page: Option<String>,
}

async fn fetch_html(url: &str) -> Result<Html> {
	let doc = reqwest::get(url).await?.text().await?;

	Ok(Html::parse_document(&doc))
}

fn parse_img_link(doc: &Html) -> Vec<String> {
	let sel = Selector::parse(".commentlist a.view_img_link").unwrap();
	let xs = doc.select(&sel);
	xs.filter_map(el_href).collect()
}

fn parse_next_page_link(doc: &Html) -> Option<String> {
	let sel = Selector::parse("a.previous-comment-page").unwrap();
	doc.select(&sel).filter_map(el_href).next()
}

async fn parse_one_page(url: &str) -> Result<JandanPage> {
	dbg!(url);

	let doc = fetch_html(url).await?;

	let link_list = parse_img_link(&doc);
	let next_page = parse_next_page_link(&doc);

	dbg!(&link_list);

	Ok(JandanPage {
		img_link_list: link_list,
		next_page,
	})
}

pub async fn create_spider_task(state: AppState) -> Result<()> {
	let img_list: Vec<String> = stream::try_unfold(Some(JANDAN_URL.to_string()), |s| async move {
		match s {
			Some(url) => {
				let x = parse_one_page(&url).await?;
				let it = stream::iter(x.img_link_list.into_iter().map(|x| Ok(x) as Result<_>));
				Ok(Some((it, x.next_page))) as Result<_>
			}
			None => Ok(None),
		}
	})
	.take(3)
	.try_flatten()
	.try_collect()
	.await?;

	write_state(&state, img_list);

	Ok(())
}
