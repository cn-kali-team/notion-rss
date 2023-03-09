<template>
  <v-container>
    <v-responsive class="d-flex align-center text-center fill-height">
      <v-navigation-drawer permanent>
        <v-list>
          <v-list-item
            :prepend-avatar="user.avatar_url"
            :title="user.name"
            subtitle=""
          ></v-list-item>
        </v-list>
        <v-divider></v-divider>
        <v-tabs v-model="tab" direction="vertical">
          <v-tab value="setting">
            <v-icon start> mdi-tune </v-icon>
            Setting
          </v-tab>
          <v-tab value="source">
            <v-icon start> mdi-rss </v-icon>
            Source
          </v-tab>
        </v-tabs>
      </v-navigation-drawer>
      <v-window v-model="tab">
        <v-window-item value="setting">
          <v-card flat>
            <v-card-text>
              <v-form ref="config_form" v-model="valid" @submit.prevent>
                <v-row no-gutters>
                  <v-col cols="12">
                    <v-text-field
                      required
                      v-model="config.notion_token"
                      :append-icon="show ? 'mdi-eye' : 'mdi-eye-off'"
                      :rules="[rules.required, rules.token]"
                      :type="show ? 'text' : 'password'"
                      name="token"
                      label="Notion Api Token"
                      hint="secret_xxx"
                      counter
                      prepend-icon="mdi-lock"
                      @click:append="show = !show"
                      @blur="$refs['config_form'].validate()"
                    ></v-text-field>
                  </v-col>
                  <v-col cols="6">
                    <v-text-field
                      required
                      :rules="[rules.required, rules.uuid]"
                      v-model="config.source_id"
                      clearable
                      label="Source Id"
                      hint="8a49af585aa844208ee085b3814e1a0d"
                      prepend-icon="mdi-rss"
                      @blur="$refs['config_form'].validate()"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="6">
                    <v-text-field
                      required
                      :rules="[rules.required, rules.uuid]"
                      v-model="config.archive_id"
                      clearable
                      label="Archive Id"
                      hint="e8f7df1fe33242a88adad7bdd793cd1e"
                      prepend-icon="mdi-archive"
                      @blur="$refs['config_form'].validate()"
                    ></v-text-field
                  ></v-col>
                </v-row>
                <v-row no-gutters>
                  <v-col cols="12">
                    <v-text-field
                      v-model="config.proxy"
                      clearable
                      label="Proxy"
                      hint="[http(s)|socks5(h)]://host:port"
                      prepend-icon="mdi-arrow-decision"
                    ></v-text-field
                  ></v-col>
                </v-row>
                <v-row no-gutters>
                  <v-col cols="6">
                    <v-switch
                      v-model="api_server_enabled"
                      color="success"
                      :label="`Enable Api Server: ${api_server_enabled}`"
                    ></v-switch>
                  </v-col>
                  <v-col cols="6">
                    <v-switch
                      v-model="config.daemon"
                      color="success"
                      :label="`Enable Daemon: ${config.daemon}`"
                    ></v-switch>
                  </v-col>
                </v-row>
                <v-row no-gutters v-show="api_server_enabled">
                  <v-col cols="5">
                    <v-text-field
                      :rules="[rules.api_server]"
                      v-model="config.api_server"
                      clearable
                      label="Api Server"
                      hint="host:port"
                      prepend-icon="mdi-server"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="5">
                    <v-text-field
                      :rules="[rules.api_server_token]"
                      v-model="config.token"
                      clearable
                      label="Api Server Token"
                      hint="token"
                      prepend-icon="mdi-security"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="2">
                    <v-tooltip text="Copy to RssHub">
                      <template v-slot:activator="{ props }">
                        <v-btn
                          size="x-small"
                          @click="onCopy"
                          v-bind="props"
                          stacked
                          prepend-icon="mdi-content-copy"
                          >Copy</v-btn
                        >
                      </template>
                    </v-tooltip>
                  </v-col>
                </v-row>
              </v-form>
              <v-divider />
              <v-card-actions>
                <v-btn
                  tile
                  color="primary"
                  :loading="update_loading"
                  @click="handle_update"
                  >Update</v-btn
                >
                <v-spacer />
                <v-btn text @click="$refs['config_form'].reset()">Reset</v-btn>
                <v-btn tile type="submit" color="primary" @click="handle_save"
                  >Save</v-btn
                >
              </v-card-actions>
            </v-card-text>
          </v-card>
        </v-window-item>
        <v-window-item value="source">
          <v-card flat>
            <v-card-text>
              <v-form ref="feed_form" v-model="valid" @submit.prevent>
                <v-row no-gutters>
                  <v-col cols="10">
                    <v-text-field
                      v-model="feed_url"
                      :rules="[rules.required, rules.url]"
                      clearable
                      label="Url"
                      hint="https://blog.kali-team.cn/index.xml"
                      prepend-icon="mdi-rss"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="2">
                    <v-tooltip text="Add Feed Source">
                      <template v-slot:activator="{ props }">
                        <v-btn
                          size="x-small"
                          @click="add_feed"
                          v-bind="props"
                          stacked
                          :loading="update_loading"
                          @blur="$refs['feed_form'].validate()"
                          prepend-icon="mdi-playlist-check"
                          >Add</v-btn
                        >
                      </template>
                    </v-tooltip>
                  </v-col>
                </v-row>
              </v-form>
              <v-divider />
            </v-card-text>
          </v-card>
        </v-window-item>
      </v-window>
      <v-main></v-main>
    </v-responsive>
  </v-container>
  <v-row justify="center">
    <v-dialog v-model="dialog" persistent width="auto">
      <v-card>
        <v-card-title class="text-h5"> Restart the app? </v-card-title>
        <v-card-text
          >You need to restart the application after modifying the
          configuration.</v-card-text
        >
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="error"
            variant="flat"
            prepend-icon="mdi-cancel"
            @click="dialog = false"
          >
            NO
          </v-btn>
          <v-btn
            color="success"
            variant="flat"
            prepend-icon="mdi-restart"
            @click="restart"
          >
            YES
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-row>
  <v-snackbar multi-line v-model="snackbar.show" timeout="3000">
    {{ snackbar.text }}
    <template v-slot:actions>
      <v-btn
        :color="snackbar.color"
        variant="flat"
        @click="snackbar.show = false"
      >
        Close
      </v-btn>
    </template>
  </v-snackbar>
</template>

<script>
import { invoke } from "@tauri-apps/api";
import { relaunch } from "@tauri-apps/api/process";
import { appWindow } from "@tauri-apps/api/window";
import { writeText } from "@tauri-apps/api/clipboard";
export default {
  name: "NotionRss",
  components: {},
  props: {},
  data() {
    return {
      show: false,
      dialog: false,
      snackbar: { show: false, text: "", color: "success" },
      api_server_enabled: false,
      valid: false,
      update_loading: false,
      feed_url: "",
      user: { avatar_url: "", name: "NotionRss" },
      config: {
        notion_token: "",
        source_id: "",
        archive_id: "",
        proxy: undefined,
        timeout: 15,
        thread: 5,
        api_server: undefined,
        token: undefined,
        daemon: false,
      },
      rules: {
        url: (v) => this.url_rules(v),
        required: (value) => !!value || "Required.",
        uuid: (v) =>
          (!!v && v.toString().length >= 32) || "Database should be uuid",
        token: (v) =>
          v.startsWith("secret_") || "Token should be start with 'secret_'",
        api_server_token: (v) => this.api_server_token_rules(v),
        api_server: (v) => this.api_server_rules(v),
      },
      tab: "setting",
    };
  },
  created() {
    this.event_listen();
    this.init_config();
  },
  methods: {
    async add_feed() {
      const { valid } = await this.$refs.feed_form.validate();
      console.log(this.feed_url);
      if (
        valid &&
        this.config.notion_token &&
        this.config.archive_id &&
        this.config.source_id
      ) {
        this.update_loading = true;
        invoke("add_feed", { url: this.feed_url, window: appWindow }).then(
          (response) => {
            console.log(response);
            this.update_loading = false;
          }
        );
      } else {
        this.snackbar = {
          text: "Check your configuration",
          show: true,
          color: "error",
        };
        this.update_loading = false;
      }
      this.update_loading = false;
    },
    async onCopy() {
      const { valid } = await this.$refs.config_form.validate();
      if (valid) {
        await writeText(
          "http://" + this.config.api_server + "/" + this.config.token + "/"
        );
        this.snackbar = {
          text: "Copy succeeded!",
          show: true,
          color: "success",
        };
      }
    },
    async url_rules(value) {
      return new Promise((resolve) => {
        try {
          new URL(value);
          return resolve(true);
        } catch (_) {
          return resolve(`URL expected`);
        }
      });
    },
    async api_server_token_rules(value) {
      return new Promise((resolve) => {
        if (!this.api_server_enabled) return resolve(true);
        if (this.api_server_enabled && !!value) return resolve(true);
        return resolve(`Required.`);
      });
    },
    async api_server_rules(value) {
      return new Promise((resolve) => {
        if (!this.api_server_enabled) {
          return resolve(true);
        } else if (
          !!value &&
          value.split(":").length == 2 &&
          !isNaN(Number(value.split(":")[1])) &&
          1024 < Number(value.split(":")[1]) &&
          Number(value.split(":")[1]) < 65535
        )
          return resolve(true);
        return resolve(
          `The service listening address is in HOST:PORT format, Port range 1024-65536.`
        );
      });
    },
    // 监听事件
    async event_listen() {
      await appWindow.listen("INFO", ({ event, payload }) => {
        console.log(event, payload);
        this.snackbar = {
          text: payload.toString(),
          show: true,
          color: "success",
        };
        this.update_loading = false;
      });
      await appWindow.listen("ERROR", ({ event, payload }) => {
        console.log(event, payload);
        this.snackbar = {
          text: payload.toString(),
          show: true,
          color: "error",
        };
        this.update_loading = false;
      });
    },
    //初始化配置
    async init_config() {
      invoke("init_config").then(async (response) => {
        this.config = response;
        if (
          this.config.api_server &&
          this.config.token &&
          this.config.notion_token &&
          this.config.archive_id &&
          this.config.source_id
        ) {
          this.api_server_enabled = true;
          invoke("run_api_server", { window: appWindow }).then((response) => {
            console.log(response);
          });
        }
        if (this.config.daemon) {
          appWindow.minimize();
        }
        await this.init_user();
      });
    },
    // 初始化用户信息
    async init_user() {
      if (
        this.config.notion_token &&
        this.config.archive_id &&
        this.config.source_id
      ) {
        invoke("init_user")
          .then((response) => {
            this.user = response;
          })
          .catch((error) => {
            this.snackbar = { text: error, show: true, color: "error" };
          });
      }
    },
    //重启APP
    async restart() {
      await relaunch();
    },
    //保存配置
    async handle_save() {
      const { valid } = await this.$refs.config_form.validate();
      if (valid) {
        if (!this.api_server_enabled) {
          this.config.api_server = undefined;
          this.config.token = undefined;
        }
        invoke("save_config", { config: this.config })
          .then((response) => {
            this.snackbar = { text: response, show: true, color: "success" };
          })
          .catch((error) => {
            this.snackbar = { text: error, show: true, color: "error" };
          });
        this.dialog = true;
      }
    },
    // 更新一次
    async handle_update() {
      const { valid } = await this.$refs.config_form.validate();
      if (
        valid &&
        this.config.notion_token &&
        this.config.archive_id &&
        this.config.source_id
      ) {
        this.update_loading = true;
        invoke("update_once", { window: appWindow }).then((response) => {
          console.log(response);
          this.update_loading = false;
        });
      } else {
        this.snackbar = {
          text: "Check your configuration",
          show: true,
          color: "error",
        };
        this.update_loading = false;
      }
    },
  },
};
</script>
