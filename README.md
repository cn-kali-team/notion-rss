[中文简体](https://blog.kali-team.cn/Notion-Rss-af7ee0e285db424e8f8349df9a92291e)

# NotionRss

Build your own RSS Feeds in [Notion](https://notion.so).

```text
███╗   ██╗ ██████╗ ████████╗██╗ ██████╗ ███╗   ██╗      ██████╗ ███████╗███████╗
████╗  ██║██╔═══██╗╚══██╔══╝██║██╔═══██╗████╗  ██║      ██╔══██╗██╔════╝██╔════╝
██╔██╗ ██║██║   ██║   ██║   ██║██║   ██║██╔██╗ ██║█████╗██████╔╝███████╗███████╗
██║╚██╗██║██║   ██║   ██║   ██║██║   ██║██║╚██╗██║╚════╝██╔══██╗╚════██║╚════██║
██║ ╚████║╚██████╔╝   ██║   ██║╚██████╔╝██║ ╚████║      ██║  ██║███████║███████║
╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝ ╚═╝  ╚═══╝      ╚═╝  ╚═╝╚══════╝╚══════╝
```

---

## Features

- Manage your RSS sources in a separate Notion page
- Enable/disable RSS sources
- Custom settings proxy request, update interval
- Automatically set icon and page title
- Read feed with different views ( unread, read, as table)
- Remove RSS items from Notion if are older than a given amount of time
- Web services are updated regularly, and background services
- Support [RssHub](https://docs.rsshub.app) browser plug-in add feed

## Setup

1. Create a new [Notion Integration](https://www.notion.so/my-integrations) and
   copy the secret code which we'll use as `--notion-token` later.
2. Duplicate the
   [template](https://kali-team.notion.site/c7f65b5d5b33470484dcf43dd6db3350)
   to your Notion.
3. Open the template and click _Share > Invite_ and search the Notion
   integration you created in Step 1 and Click **Invite**.
4. Click on _Sources_ page and do the same actions as in the previous step.
5. Now you have the api token of notion, start with `secret_` a string,
   it is used for the `--notion-token` parameter.

   > To find the source database ID, open the Source page. You'll get a URL
   > like this: https://www.notion.so/{database_id}?v={view_id}. To find the
   > archive database ID, open the main page of the template. Highlight the table
   > and click on "dots" button and select _Open as page_ option. Copy the
   > database ID from the URL in the same way you did for the source database
   > ID.
   > Example: my source-id: 8a49af585aa844208ee085b3814e1a0d
   > my archive-id: e8f7df1fe33242a88adad7bdd793cd1e

- The application accepts the source database ID and archive database ID as parameters.

```bash
➜  notion-rss --notion-token secret_xxx --source-id 8a49af585aa844208ee085b3814e1a0d --archive-id e8f7df1fe33242a88adad7bdd793cd1e
```

## Usage

```text
➜ notion-rss --help
Usage: notion-rss [--notion-token <notion-token>] [--source-id <source-id>] [--archive-id <archive-id>] [-f <file>] [-c <config>] [--proxy <proxy>] [--timeout <timeout>] [--deleted] [--thread <thread>] [--webhook <webhook>] [--api-server <api-server>] [--token <token>] [--daemon] [--cli]

notion-rss

Options:
  --notion-token    the notion api token
  --source-id       the source database id
  --archive-id      the archive database id
  -f, --file        add feed from url or file
  -c, --config      read the config from the file
  --proxy           proxy to use for requests
                    (ex:[http(s)|socks5(h)]://host:port)
  --timeout         set request timeout.
  --deleted         deleted old archive
  --thread          number of concurrent threads.
  --webhook         send results to webhook server
                    (ex:https://host:port/webhook)
  --api-server      start a web API service (ex:127.0.0.1:8080)
  --token           api Router authentication
  --daemon          api background service
  --cli             cli mode
  --help            display usage information

```

## Example

### Add feed from file

- One feed link per line.

```bash
➜ cat /home/kali-team/feed.txt | head
https://blog.kali-team.cn/index.xml
https://www.exploit-db.com/rss.xml

➜ notion-rss --notion-token secret_xxx --source-id 8a49af585aa844208ee085b3814e1a0d --archive-id e8f7df1fe33242a88adad7bdd793cd1e --file /home/kali-team/feed.txt
Update succeeded:  [Title:三米前有蕉皮]  [Link:Some("https://blog.kali-team.cn/index.xml")]  [Status:Done] 
Update succeeded:  [Title:exploit-db]  [Link:Some("https://www.exploit-db.com/rss.xml")]  [Status:Done]
```

### Deleted old archive

- Edit the `Deleted` properties of the Archive database.
- Like: `dateBetween(now(), prop("Last Update"), "years") >= 3`, Filter those released three years ago.
- `empty(prop("ForeignKey"))`, Filter the Archive of the deleted feed.
- Re-execute the deletion operation after changing the filter expression.
- If there are a large number of archives, it is recommended to delete them manually, because the api has a rate limit.

```bash
➜  notion-rss --notion-token secret_xxx --source-id 8a49af585aa844208ee085b3814e1a0d --archive-id e8f7df1fe33242a88adad7bdd793cd1e --deleted
Deleted succeeded: Hello World
Deleted succeeded: Debugging
```

### Web Server

- It is started as a service and will be updated regularly in the background.
- Update every 4 hours.
- And you can add feeds through the browser plug-in of RssHub.
- To prevent CSRF attacks, you need to specify the `--token` parameter and set a token as a random route.

```bash
➜  notion-rss --notion-token secret_xxx --source-id 8a49af585aa844208ee085b3814e1a0d --archive-id e8f7df1fe33242a88adad7bdd793cd1e --api-server 127.0.0.1:8080 --token 21a2b7047d4de8076de462724daf8f8f
Copy the URL to your RssHub browser plug-in configuration
Configure any of the following
[Tiny Tiny RSS, Miniflux, FreshRSS, Nextcloud News, InoReader, FeedBin]
API service has been started: http://127.0.0.1:8080/21a2b7047d4de8076de462724daf8f8f/
```

- Copy the `http://127.0.0.1:8080/21a2b7047d4de8076de462724daf8f8f/` and fill it in any one of the support list of
  RssHub.
- Click the browser plug-in to add a feed.
- The page will close automatically after 5 seconds.

### Specify profile

- Save the command line configuration in the configuration file, and specify the configuration file path each time.

```bash
➜ notion-rss --config /home/kali-team/.config/notion-rss/config.yaml

```

- If the configuration is valid, it will be automatically saved to the configuration file.

```yaml
config:
  notion_token: secret_xxx
  source_id: 8a49af58-5aa8-4420-8ee0-85b3814e1a0d
  archive_id: e8f7df1f-e332-42a8-8ada-d7bdd793cd1e
  proxy: null
  timeout: 15
  thread: 5
  webhook: null
  api_server: 127.0.0.1:8080
  token: 21a2b7047d4de8076de462724daf8f8f
  daemon: false
```

## Cli Mode

- Command line mode using the `--cli` parameter, The default is graphical interface.

## Docker

- [docker](https://hub.docker.com/r/kaliteam/notion-rss)

```bash
docker run --rm -it \
--env NR_NOTION_TOKEN=secret_xxx \
--env NR_SOURCE_ID=8a49af58-5aa8-4420-8ee0-85b3814e1a0d \
--env NR_ARCHIVE_ID=e8f7df1f-e332-42a8-8ada-d7bdd793cd1e \
--env NR_API_SERVER=127.0.0.1:8080 \
--env NR_TOKEN=2a7b648abbc89a966d2b295f4d7780f4 \
--env NR_DAEMON="true" \
kaliteam/notion-rss:latest
```

- use [docker-compose.yml](docker-compose.yml)
- run `docker-compose up`

```yaml
version: '3'

services:
  notion-rss:
    image: kaliteam/notion-rss:latest
    ports:
      - "9527:9527"
    environment:
      NR_NOTION_TOKEN: ${NR_NOTION_TOKEN}
      NR_SOURCE_ID: ${NR_SOURCE_ID}
      NR_ARCHIVE_ID: ${NR_ARCHIVE_ID}
      NR_API_SERVER: ${NR_API_SERVER}
      NR_TOKEN: ${NR_TOKEN}
      NR_PROXY: ${NR_PROXY}
      NR_DAEMON: ${NR_DAEMON}
      NR_TIMEOUT: ${NR_TIMEOUT}
      NR_HOUR: ${NR_HOUR}
```