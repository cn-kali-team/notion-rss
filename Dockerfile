FROM alpine/curl
WORKDIR /NOTION_RSS
ENV TZ=Asia/Shanghai \
    TIMEOUT=15 \
    HOUR=4 \
    API_SERVER="127.0.0.1:9527" \
    TOKEN=""
RUN wget $(curl -sL https://api.github.com/repos/cn-kali-team/notion-rss/releases/latest | jq -r '.assets[]|select(.name |startswith("notion-rss-cli_"))|select(.name |endswith("unknown-linux-musl.tar.gz")).browser_download_url')
RUN tar xvfz $(ls notion-rss-cli_*_x86_64-unknown-linux-musl.tar.gz)
RUN chmod +x /NOTION_RSS/notion-rss-cli
EXPOSE 9527 9527
ENTRYPOINT /NOTION_RSS/notion-rss-cli --notion-token "${NOTION_TOKEN}" --source-id "${SOURCE_ID}" --archive-id "${ARCHIVE_ID}" --timeout "${TIMEOUT}" --hour "${HOUR}" --daemon --api-server "${API_SERVER}" --token "${TOKEN}"