# 盔甲鸟

![灰甲鸟](https://media.52poke.com/wiki/thumb/archive/3/35/20140413164313%21227Skarmory.png/120px-227Skarmory.png)

今天喝水了吗？

# 运行

制作docker镜像：

```fish
docker build -f docker/rust.Dockerfile -t skarmory .
```

准备好配置文件，假如叫`myconf.toml`，内容如下：

```toml
salt = "随便" # 密码加盐，此处选填；一旦确定，以后都不能改。

[database]
host = "localhost"
port = 5432
user = "postgres"
password = "postgres"
database = "postgres"
```

完毕后即可启动镜像：

```fish
docker run -p 3000:3000 -v $PWD/myconf.toml:/opt/config/config.toml skarmory
```

# 协议

AGPL v3
