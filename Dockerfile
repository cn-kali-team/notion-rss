FROM alpine/curl
WORKDIR /NOTION_RSS
ENV TZ=Asia/Shanghai
RUN wget $(curl -sL https://api.github.com/repos/cn-kali-team/notion-rss/releases/latest | jq -r '.assets[]|select(.name |startswith("notion-rss-cli_"))|select(.name |endswith("unknown-linux-musl.tar.gz")).browser_download_url')
RUN tar xvfz $(ls notion-rss-cli_*_x86_64-unknown-linux-musl.tar.gz)
RUN chmod +x /NOTION_RSS/notion-rss-cli
ENTRYPOINT ["/NOTION_RSS/notion-rss-cli"]