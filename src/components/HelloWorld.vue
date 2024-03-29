<template>
  <v-container>
    <v-responsive class="d-flex align-center text-center fill-height">
      <v-navigation-drawer permanent>
        <v-list>
          <v-list-item
            :prepend-avatar="user.avatar_url"
            :title="user.name"
          ></v-list-item>
        </v-list>
        <v-divider></v-divider>
        <v-tabs v-model="tab" direction="vertical">
          <v-tab value="setting">
            <v-icon start> mdi-tune </v-icon>
            {{ $vuetify.locale.t("tab.setting") }}
          </v-tab>
          <v-tab value="tools">
            <v-icon start> mdi-hammer-screwdriver </v-icon>
            {{ $vuetify.locale.t("tab.tools") }}
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
                      :label="$vuetify.locale.t('label.notion_token')"
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
                      :label="$vuetify.locale.t('label.source_id')"
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
                      :label="$vuetify.locale.t('label.archive_id')"
                      hint="e8f7df1fe33242a88adad7bdd793cd1e"
                      prepend-icon="mdi-archive"
                      @blur="$refs['config_form'].validate()"
                    ></v-text-field
                  ></v-col>
                </v-row>
                <v-row no-gutters>
                  <v-col cols="6">
                    <v-text-field
                      v-model="config.proxy"
                      clearable
                      :label="$vuetify.locale.t('label.proxy')"
                      hint="[http(s)|socks5(h)]://host:port"
                      prepend-icon="mdi-arrow-decision"
                    >
                    </v-text-field
                  ></v-col>
                  <v-col cols="6">
                    <v-slider
                      v-model="config.hour"
                      thumb-label="always"
                      step="1"
                      :hint="$vuetify.locale.t('text.scheduled')"
                      :max="24"
                      :min="1"
                      :label="$vuetify.locale.t('label.scheduled')"
                      prepend-icon="mdi-clock-outline"
                    >
                      <template v-slot:append>
                        <v-text-field
                          v-model="config.hour"
                          type="number"
                          style="width: 100px"
                          density="compact"
                          hide-details
                          variant="outlined"
                        ></v-text-field> </template></v-slider
                  ></v-col>
                </v-row>
                <v-row no-gutters>
                  <v-col cols="6">
                    <v-switch
                      v-model="api_server_enabled"
                      color="success"
                      :label="
                        $vuetify.locale.t('label.api_server_enabled') +
                        `: ${api_server_enabled}`
                      "
                    ></v-switch>
                  </v-col>
                  <v-col cols="6">
                    <v-switch
                      v-model="config.daemon"
                      color="success"
                      :label="
                        $vuetify.locale.t('label.daemon') + ` ${config.daemon}`
                      "
                    ></v-switch>
                  </v-col>
                </v-row>
                <v-row no-gutters v-show="api_server_enabled">
                  <v-col cols="5">
                    <v-text-field
                      :rules="[rules.api_server]"
                      v-model="config.api_server"
                      clearable
                      :label="$vuetify.locale.t('label.api_server')"
                      hint="host:port"
                      prepend-icon="mdi-server"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="5">
                    <v-text-field
                      :rules="[rules.api_server_token]"
                      v-model="config.token"
                      clearable
                      :label="$vuetify.locale.t('label.token')"
                      hint="token"
                      prepend-icon="mdi-security"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="2">
                    <v-tooltip :text="$vuetify.locale.t('tooltip.rsshub')">
                      <template v-slot:activator="{ props }">
                        <v-btn
                          size="x-small"
                          @click="onCopy"
                          v-bind="props"
                          stacked
                          prepend-icon="mdi-content-copy"
                        >
                          {{ $vuetify.locale.t("btn.copy") }}
                        </v-btn>
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
                >
                  {{ $vuetify.locale.t("btn.update") }}</v-btn
                >
                <v-spacer />
                <v-btn text @click="$refs['config_form'].reset()">{{
                  $vuetify.locale.t("btn.reset")
                }}</v-btn>
                <v-btn tile type="submit" color="primary" @click="handle_save">
                  {{ $vuetify.locale.t("btn.save") }}
                </v-btn>
              </v-card-actions>
            </v-card-text>
          </v-card>
        </v-window-item>
        <v-window-item value="tools">
          <v-card flat>
            <v-card-text>
              <v-form ref="feed_form" v-model="valid" @submit.prevent>
                <v-row no-gutters>
                  <v-col cols="8">
                    <v-text-field
                      v-model="feed_url"
                      :rules="[rules.required, rules.url]"
                      clearable
                      label="Url"
                      hint="https://blog.kali-team.cn/index.xml"
                      prepend-icon="mdi-rss"
                    ></v-text-field
                  ></v-col>
                  <v-col>
                    <v-tooltip :text="$vuetify.locale.t('tooltip.add_feed')">
                      <template v-slot:activator="{ props }">
                        <v-btn
                          size="large"
                          @click="add_feed"
                          v-bind="props"
                          :loading="update_loading"
                          prepend-icon="mdi-playlist-check"
                        >
                          {{ $vuetify.locale.t("btn.add") }}</v-btn
                        >
                      </template>
                    </v-tooltip>
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="8">
                    <v-file-input
                      :label="$vuetify.locale.t('label.feed_file')"
                      prepend-icon="mdi-file-xml-box"
                      show-size
                      counter
                      accept=".opml,.txt,.xml"
                      v-model="feed_file"
                      @change="load_file"
                    ></v-file-input>
                  </v-col>
                  <v-col>
                    <v-tooltip :text="$vuetify.locale.t('tooltip.import_feed')">
                      <template v-slot:activator="{ props }">
                        <v-btn
                          size="large"
                          @click="import_feed"
                          v-bind="props"
                          :loading="update_loading"
                          prepend-icon="mdi-file-import"
                        >
                          {{ $vuetify.locale.t("btn.import") }}
                        </v-btn>
                      </template>
                    </v-tooltip>
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="8">
                    <v-progress-linear
                      v-show="show_progress_bar"
                      striped
                      height="25"
                      v-model="make_progress_bar"
                      color="primary"
                    >
                      <template v-slot:default="{ value }">
                        <strong>
                          {{ $vuetify.locale.t("text.importing") }}
                          {{ Math.ceil(value) }}%</strong
                        >
                      </template>
                    </v-progress-linear>
                  </v-col>
                </v-row>
                <v-row>
                  <v-col cols="4">
                    <v-select
                      v-model="lang"
                      @update:model-value="changeLocale"
                      prepend-icon="mdi-translate"
                      :label="$vuetify.locale.t('label.language')"
                      :items="['en', 'zhHans']"
                      variant="solo"
                    ></v-select>
                  </v-col>
                </v-row>
              </v-form>
              <v-divider />
              <v-btn
                flat
                @click="update_app"
                :loading="update_loading"
                prepend-icon="mdi-update"
              >
                {{ $vuetify.locale.t("btn.update_app") }}
              </v-btn>
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
        <v-card-title class="text-h5">
          {{ $vuetify.locale.t("text.restart_app") }}</v-card-title
        >
        <v-card-text>{{
          $vuetify.locale.t("text.is_restart_app")
        }}</v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="error"
            variant="flat"
            prepend-icon="mdi-cancel"
            @click="dialog = false"
          >
            {{ $vuetify.locale.t("btn.no") }}
          </v-btn>
          <v-btn
            color="success"
            variant="flat"
            prepend-icon="mdi-restart"
            @click="restart"
          >
            {{ $vuetify.locale.t("btn.yes") }}
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
        {{ $vuetify.locale.t("btn.close") }}
      </v-btn>
    </template>
  </v-snackbar>
</template>

<script>
import { invoke } from "@tauri-apps/api";
import { relaunch } from "@tauri-apps/api/process";
import { appWindow } from "@tauri-apps/api/window";
import { writeText } from "@tauri-apps/api/clipboard";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { useLocale } from "vuetify";
export default {
  name: "NotionRss",
  components: {},
  props: {},
  data() {
    return {
      lang: window.localStorage.getItem("lang") || "en",
      progress_bar: { total: 0, progress: 0 },
      show_progress_bar: false,
      show: false,
      dialog: false,
      snackbar: { show: false, text: "", color: "success" },
      api_server_enabled: false,
      valid: false,
      update_loading: false,
      feed_url: "",
      feed_file: [],
      content: null,
      user: { avatar_url: "favicon.ico", name: "NotionRss" },
      config: {
        notion_token: "",
        source_id: "",
        archive_id: "",
        proxy: undefined,
        timeout: 15,
        hour: 4,
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
  computed: {
    // whenever question changes, this function will run
    make_progress_bar() {
      return (this.progress_bar.progress / this.progress_bar.total) * 100;
    },
  },
  setup() {
    const { t } = useLocale();
    return {
      t,
    };
  },
  created() {
    this.event_listen();
    this.init_config();
  },
  methods: {
    changeLocale(locale) {
      this.$vuetify.locale.current = locale;
      window.localStorage.setItem("lang", locale);
    },
    async update_app() {
      try {
        this.update_loading = true;
        const { shouldUpdate, manifest } = await checkUpdate();
        console.log(manifest);
        if (shouldUpdate) {
          // display dialog
          await installUpdate();
          // install complete, restart the app
          await relaunch();
        }
      } catch (error) {
        this.snackbar = {
          text: error.toString(),
          show: true,
          color: "error",
        };
      }
      this.update_loading = false;
    },
    async add_feed() {
      if (
        !!this.feed_url &&
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
      }
    },
    load_file() {
      if (this.feed_file) {
        const reader = new FileReader();
        let files = this.feed_file[0];
        reader.onload = (res) => {
          this.content = res.target.result;
        };
        reader.onerror = (err) => console.log(err);
        reader.readAsText(files);
      } else {
        this.snackbar = {
          text: "Check your configuration",
          show: true,
          color: "error",
        };
      }
    },
    async import_feed() {
      if (
        !!this.content &&
        this.config.notion_token &&
        this.config.archive_id &&
        this.config.source_id
      ) {
        this.show_progress_bar = true;
        this.update_loading = true;
        invoke("import_feed", {
          content: this.content,
          window: appWindow,
        }).then((response) => {
          console.log(response);
          this.update_loading = false;
        });
      } else {
        this.snackbar = {
          text: "Check your configuration",
          show: true,
          color: "error",
        };
      }
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
      await appWindow.listen("PROGRESS", ({ event, payload }) => {
        console.log(event, payload);
        this.progress_bar = payload;
        if (this.progress_bar.total == this.progress_bar.progress) {
          this.snackbar = {
            text: "Done",
            show: true,
            color: "success",
          };
        }
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
      this.changeLocale(this.lang);
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
            if (response) {
              this.user = response;
            } else {
              this.snackbar = {
                text: "Check your configuration",
                show: true,
                color: "error",
              };
            }
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
