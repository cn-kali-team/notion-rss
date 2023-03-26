import { zhHans } from "vuetify/locale";
export default {
  $vuetify: { ...zhHans },
  btn: {
    close: "关闭",
    save: "保存",
    yes: "是",
    no: "否",
    copy: "复制",
    update: "更新",
    add: "添加",
    import: "导入",
    update_app: "更新应用",
    reset:"重置"
  },
  tab: {
    setting: "Notion设置",
    tools: "工具",
  },
  label: {
    source_id: "订阅源的数据库ID",
    notion_token: "Notion集成API的Token",
    archive_id: "文章归档的数据库ID",
    proxy: "代理",
    scheduled: "定时",
    api_server_enabled: "是否开启API服务",
    api_server: "API的监听地址和监听端口",
    token: "API服务的Token",
    feed_file: "选择一个OPML文件",
    daemon: "是否后台运行：",
    language:"语言"
  },
  tooltip: {
    rsshub: "复制到RssHub浏览器插件",
    add_feed: "添加订阅源",
    import_feed: "从文件中导入订阅源",
  },
  text: {
    importing: "导入进度：",
    scheduled:"每多少小时更新一次",
    restart_app: "是否重新打开应用? ",
    is_restart_app: "修改设置后必须重新启动应用才可以生效。",
  },
};
