javascript: function iptxt() {
  try {
    if (!document.body) throw 0;
    const body = document.getElementsByTagName("body")[0];
    const div = document.createElement("div");
    div.innerHTML = `
    <div style="font-size:14px;color:rgba(0,0,0,0.65);box-shadow: 0 2px 12px 0 rgba(0,0,0,0.1) ;background-color: #fff;border-radius: 5px;border: 1px solid #ebebeb;left:20px;top:20px;position: fixed;z-index: 1000000">
    <div style="display: flex;justify-content: flex-end;margin-right: 8px;">
      <div style="color:rgb(24, 36, 127);cursor: pointer;margin-left: 8px;margin-top: 4px;font-size: 12px;margin-bottom: 4px;display: none" id="icon-bottom">NotionRss</div>
      <div style="cursor: pointer;font-size: 12px;margin-left: 8px;margin-top: 2px;color:rgb(96, 98, 102);" id="icon-hide">Hide</div>
      <div style="cursor: pointer;font-size: 12px;margin-left: 2px;margin-top: 2px;color:rgb(96, 98, 102);" id="icon-close">Close</div>
    </div>
    <div id="plugs_content" style="padding-bottom: 10px;">
      <div style="margin-left: 4px;border-radius: 2px;display: inline-block;padding: 8px;padding-right: 40px;">
        <div style="margin-bottom: 4px;display: flex">
          <div style="margin-right: 6px;white-space: nowrap">URL:</div>
          <input type="text" id="rss">
        </div>
      </div>
    <div id="add_feed" style="cursor: pointer;display: flex;margin-top: 8px;border-top:1px solid #ebebeb;justify-content: flex-end;padding-right: 4px;">
          Add
    </div>
    </div>
  </div>`;
    body.appendChild(div);
    const hide_icon = document.getElementById("icon-hide");
    const close_icon = document.getElementById("icon-close");
    const bottom_icon = document.getElementById("icon-bottom");
    const content = document.getElementById("plugs_content");
    const copy = document.getElementById("add_feed");
    hide_icon.onclick = () => {
      content.style.display = "none";
      hide_icon.style.display = "none";
      close_icon.style.display = "none";
      bottom_icon.style.display = "block";
    };
    bottom_icon.onclick = () => {
      content.style.display = "block";
      hide_icon.style.display = "block";
      close_icon.style.display = "block";
      bottom_icon.style.display = "none";
    };
    close_icon.onclick = () => {
      div.remove();
    };
    copy.onclick = () => {
      const rss = document.getElementById("rss").value;
      if (!rss) return;
      window.open(
        "http://127.0.0.1:8080/14fe27c312f2828deb73bb1c7bfd92dc/?subscribe_to=" +
          rss
      );
    };
  } catch (e) {
    alert("Please wait until the page has loaded." + e);
  }
}
iptxt();
void 0;
