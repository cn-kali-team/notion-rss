/* CONFIGURATION STARTS HERE */

const NOTION_TOKEN = "secret_xxx";
const SOURCE_ID = "8a49af58-5aa8-4420-8ee0-85b3814e1a0d";
const API_TOKEN = "14fe27c312f2828deb73bb1c7bfd92dc";
const pages = `
<!DOCTYPE html>
<html>
<head>
	<title>NOTION-RSS</title>
	<meta charset="UTF-8">
	<script>
		window.onload = function() {
			var count = 5;
			var countdown = setInterval(function() {
				document.getElementById('countdown').innerHTML = count;
				count--;
				if (count < 0) {
					clearInterval(countdown);
					window.location.href="about:blank";
					window.close();
				}
			}, 1000);
		};
	</script>
</head>
<body>
	<div style="display: flex; justify-content: center; align-items: center;">
	    <h1>Kali-Team</h1>
	</div>
	<div style="display: flex; justify-content: center; align-items: center;">
	    <p>The page will automatically close in <span id="countdown">5</span> seconds.</p>
	</div>
</body>
</html>
`;

addEventListener("fetch", (event) => {
  event.respondWith(fetchAndApply(event.request));
});

async function filter_from_database(rss_url) {
  const options = {
    method: "POST",
    headers: {
      "Notion-Version": "2022-02-22",
      "Content-Type": "application/json",
      Authorization: "Bearer " + NOTION_TOKEN,
    },
    body:
      '{"filter":{"or":[{"property":"Link","rich_text":{"equals":"' +
      rss_url +
      '"}}]}}',
  };
  let response = await fetch(
    "https://api.notion.com/v1/databases/" + SOURCE_ID + "/query",
    options
  );
  let body = await response.json();
  let results = body.results;
  let title = null;
  if (results.length > 0) {
    let titles = results[0].properties.Title.title;
    if (titles.length > 0) {
      title = titles[0].plain_text;
    } else {
      title = "";
    }
  }
  return title;
}
async function create_page(rss_url) {
  const options = {
    method: "POST",
    headers: {
      "Notion-Version": "2022-02-22",
      "Content-Type": "application/json",
      Authorization: "Bearer " + NOTION_TOKEN,
    },
    body:
      '{"parent":{"database_id":"' +
      SOURCE_ID +
      '"},"properties":{"Link":{"url":"' +
      rss_url +
      '"},"Enabled":{"checkbox":true}}}',
  };
  let response = await fetch("https://api.notion.com/v1/pages", options);
  let body = await response.json();
  let properties = body.properties;
  let title = null;
  if (properties.length > 0) {
    let titles = properties.Title.title;
    if (titles.length > 0) {
      title = titles[0].plain_text;
      rss_url = rss_url + title;
    } else {
      rss_url = null;
    }
  }
  return rss_url;
}
async function add_subscribe(searchParams) {
  let rss_url = searchParams.get("uri");
  let title = await filter_from_database(rss_url);
  let msg = "Submitted Failed: " + rss_url;
  if (title == null) {
    rss_url = await create_page(rss_url);
    msg = "Submitted Successfully :" + rss_url;
  } else {
    msg = "The feed already exists as :" + title;
  }
  let new_pages = pages.replace("Kali-Team", msg);
  return new_pages;
}
const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods":
    "GET,POST,PUT,PATCH,TRACE,DELETE,HEAD,OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
  "Access-Control-Allow-Credentials": "True",
  "Access-Control-Max-Age": "1728000",
};

function handleOptions(request) {
  if (
    request.headers.get("Origin") !== null &&
    request.headers.get("Access-Control-Request-Method") !== null &&
    request.headers.get("Access-Control-Request-Headers") !== null
  ) {
    // Handle CORS pre-flight request.
    for (const [k, v] of corsHeaders.entries()) {
      request.headers.set(k, v);
    }
    return new Response(null, {
      headers: request.headers,
    });
  } else {
    // Handle standard OPTIONS request.
    return new Response(null, {
      headers: {
        Allow: "GET,POST,PUT,PATCH,TRACE,DELETE,HEAD,OPTIONS",
      },
    });
  }
}

async function fetchAndApply(request) {
  if (request.method === "OPTIONS") {
    return handleOptions(request);
  }
  let url = new URL(request.url);
  if (url.pathname === "/" + API_TOKEN + "/bookmarklet") {
    let response = new Response(await add_subscribe(url.searchParams));
    response.headers.set("Content-Type", "text/html;charset=UTF-8");
    return response;
  }
  let response;
  let err_body = pages.replace(
    "Kali-Team",
    "Path error, please check Token parameter."
  );
  response = new Response(err_body);
  response.headers.set("Content-Type", "text/html;charset=UTF-8");
  return response;
}
