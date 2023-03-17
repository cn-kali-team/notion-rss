import { en } from "vuetify/locale";

export default {
  $vuetify: { ...en },
  btn: {
    close: "Close",
    save: "Save",
    yes: "Yes",
    no: "No",
    copy: "Copy",
    update: "Update",
    add: "Add",
    import: "Import",
    update_app: "Update App",
    reset:"Reset"
  },
  tab: {
    setting: "Setting",
    tools: "Tools",
  },
  label: {
    source_id: "Source Id",
    notion_token: "Notion Api Token",
    archive_id: "Archive Id",
    proxy: "Proxy",
    api_server_enabled: "Enable Api Server",
    api_server: "Api Server",
    token: "Api Server Token",
    feed_file: "Select an OPML file",
    daemon: "Daemon:",
    language:"Language"
  },
  tooltip: {
    rsshub: "Copy to RssHub",
    add_feed: "Add Feed Source",
    import_feed: "Import source from opml file",
  },
  text: {
    importing: "Importing:",
    restart_app: "Restart the app? ",
    is_restart_app:
      "You need to restart the application after modifying the configuration.",
  },
};
