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
          <v-tab value="archive">
            <v-icon start> mdi-archive </v-icon>
            Archive
          </v-tab>
        </v-tabs>
      </v-navigation-drawer>
      <v-window v-model="tab">
        <v-window-item value="setting">
          <v-card flat>
            <v-card-text>
              <v-form ref="config_form" v-model="valid">
                <v-row no-gutters>
                  <v-col cols="12">
                    <v-text-field
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
                    ></v-text-field>
                  </v-col>
                  <v-col cols="6">
                    <v-text-field
                      :rules="[rules.required]"
                      v-model="config.source_id"
                      clearable
                      label="Source Id"
                      hint="8a49af585aa844208ee085b3814e1a0d"
                      prepend-icon="mdi-rss"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="6">
                    <v-text-field
                      :rules="[rules.required]"
                      v-model="config.archive_id"
                      clearable
                      label="Archive Id"
                      hint="e8f7df1fe33242a88adad7bdd793cd1e"
                      prepend-icon="mdi-archive"
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
                  <v-col cols="6">
                    <v-text-field
                      :rules="[rules.api_server]"
                      v-model="config.api_server"
                      clearable
                      label="Api Server"
                      hint="host:port"
                      prepend-icon="mdi-server"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="6">
                    <v-text-field
                      :rules="[rules.api_server_token]"
                      v-model="config.token"
                      clearable
                      label="Api Server Token"
                      hint="token"
                      prepend-icon="mdi-security"
                    ></v-text-field
                  ></v-col>
                </v-row>
              </v-form>
            </v-card-text>
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
              <v-btn tile color="primary" @click="handle_save">Save</v-btn>
            </v-card-actions>
          </v-card>
        </v-window-item>
        <v-window-item value="source">
          <v-card flat>
            <v-card-text> </v-card-text>
          </v-card>
        </v-window-item>
        <v-window-item value="archive">
          <v-card flat>
            <v-card-text> </v-card-text>
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
  <v-snackbar v-model="snackbar.show" timeout="3000">
    {{ snackbar.text }}

    <template v-slot:actions>
      <v-btn color="blue" variant="text" @click="snackbar.show = false">
        Close
      </v-btn>
    </template>
  </v-snackbar>
</template>

<script>
import { invoke } from "@tauri-apps/api";
import { relaunch } from "@tauri-apps/api/process";
import { appWindow } from "@tauri-apps/api/window";
export default {
  name: "NotionRss",
  components: {},
  props: {},
  data() {
    return {
      show: false,
      dialog: false,
      snackbar: { show: false, text: "" },
      api_server_enabled: false,
      valid: true,
      update_loading: false,
      user:{avatar_url:"",name:"NotionRss"},
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
      token: "",
      rules: {
        required: (value) => !!value || "Required.",
        token: (v) =>
          v.startsWith("secret_") || "Token should be start with 'secret_'",
        api_server_token: (v) =>
          (this.api_server_enabled && !!v) || `Required.`,
        api_server: (v) =>
          (this.api_server_enabled && v.split(":").length == 2) ||
          `The service listening address is in HOST:PORT format`,
      },
      tab: "setting",
    };
  },
  created() {
    this.event_listen();
    this.init_config();
    this.init_user();
  },
  methods: {
    async event_listen() {
      await appWindow.listen("PROGRESS", ({ event, payload }) => {
        console.log(event, payload),
          (this.snackbar.text = payload.toString()),
          (this.snackbar.show = true);
        this.update_loading = false;
      });
    },
    async init_config() {
      invoke("init_config").then((response) => {
        this.config = response;
        if (this.config.api_server && this.config.token) {
          this.api_server_enabled = true;
          invoke("run_api_server", { window: appWindow }).then((response) => {
            console.log(response);
          });
        }
        if (this.config.daemon) {
          appWindow.minimize();
        }
      });
    },
    async init_user() {
      invoke("init_user").then((response) => {
          console.log(response)
          this.user = response;
      });
    },
    async restart() {
      await relaunch();
    },
    async handle_save() {
      if (this.$refs["config_form"].validate()) {
        if (!this.api_server_enabled) {
          this.config.api_server = undefined;
          this.config.token = undefined;
        }
        invoke("save_config", { config: this.config }).then((response) => {
          this.snackbar.text = response;
          this.snackbar.show = true;
        });
        this.dialog = true;
      }
    },
    // 更新一次
    async handle_update() {
      if (this.$refs["config_form"].validate()) {
        this.update_loading = true;
        invoke("update_once", { window: appWindow }).then((response) => {
          console.log(response);
        });
      }
    },
  },
};
</script>
