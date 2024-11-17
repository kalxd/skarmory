use crate::state::AppState;
use crate::{error::Result, state::write_state};
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
	let doc = fetch_html(url).await?;

	let link_list = parse_img_link(&doc);
	let next_page = parse_next_page_link(&doc);

	Ok(JandanPage {
		img_link_list: link_list,
		next_page,
	})
}

pub async fn create_spide_task(state: AppState) -> Result<()> {
	let page = parse_one_page(&JANDAN_URL).await?;

	let mut total = page.img_link_list;

	if let Some(next_url) = page.next_page {
		let page = parse_one_page(&next_url).await?;
		total.extend(page.img_link_list);
	}

	write_state(&state, total);

	Ok(())
}
