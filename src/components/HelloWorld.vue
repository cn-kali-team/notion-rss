<template>
  <v-container class="fill-height">
    <v-responsive class="d-flex align-center text-center fill-height">
      <v-navigation-drawer permanent>
        <v-list>
          <v-list-item
            prepend-avatar="https://www.gravatar.com/avatar/bc985422e4fd51abac5af2691ee33a93"
            title="Kali-Team"
            subtitle="root@kali-team.cn"
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
                      v-model="config.source_id"
                      clearable
                      label="Source Id"
                      hint="8a49af585aa844208ee085b3814e1a0d"
                      prepend-icon="mdi-rss"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="6">
                    <v-text-field
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
                    <v-text-field
                      v-model="config.api_server"
                      clearable
                      label="Api Server"
                      hint="host:port"
                      prepend-icon="mdi-server"
                    ></v-text-field
                  ></v-col>
                  <v-col cols="6">
                    <v-text-field
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
            <v-divider class="mt-5" />
            <v-card-actions>
              <v-spacer />
              <v-btn text @click="$refs['config_form'].reset()">Reset</v-btn>
              <v-btn
                tile
                color="primary"
                :loading="save_loading"
                @click="handle_save"
                >Save</v-btn
              >
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
      <v-main style="height: 100px"></v-main>
    </v-responsive>
  </v-container>
</template>

<script>
import { invoke } from "@tauri-apps/api";
export default {
  name: "NotionRss",
  components: {},
  props: {},
  data() {
    return {
      show: false,
      valid: true,
      save_loading: false,
      config: {
        notion_token: "",
        source_id: "",
        archive_id: "",
        proxy: "",
        timeout: 15,
        thread: 5,
        api_server: "",
        token: "",
        daemon: false,
      },
      token: "",
      rules: {
        required: (value) => !!value || "Required.",
        token: (v) =>
          v.startsWith("secret_") || "Token should be start with 'secret_'",
        emailMatch: () => `The email and password you entered don't match`,
      },
      tab: "setting",
    };
  },
  created() {
    this.init_config();
  },
  methods: {
    async init_config() {
      invoke("init_config").then((response) => {
        console.log(response);
        this.config = response;
      });
    },
    handle_save() {
      if (this.$refs["config_form"].validate()) {
        this.btn_loading = true;
      }
    },
  },
};
</script>
