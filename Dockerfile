FROM alpine/curl
WORKDIR /NOTION_RSS
ENV TZ=Asia/Shanghai
ARG DOWNLOAD_URL
RUN wget "${DOWNLOAD_URL}"
RUN tar xvfz $(ls notion-rss-cli_*_x86_64-unknown-linux-musl.tar.gz)
RUN rm $(ls notion-rss-cli_*_x86_64-unknown-linux-musl.tar.gz)
RUN chmod +x /NOTION_RSS/notion-rss-cli
ENTRYPOINT ["/NOTION_RSS/notion-rss-cli"]