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
              <v-form ref="task_form" v-model="valid">
                <v-row no-gutters>
                  <v-col cols="12">
                    <v-text-field
                      v-model="token"
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
                  <v-text-field
                    clearable
                    label="Source Id"
                    hint="8a49af585aa844208ee085b3814e1a0d"
                    prepend-icon="mdi-rss"
                  ></v-text-field>
                  <v-text-field
                    clearable
                    label="Archive Id"
                    hint="e8f7df1fe33242a88adad7bdd793cd1e"
                    prepend-icon="mdi-archive"
                  ></v-text-field>
                </v-row>
              </v-form>
            </v-card-text>
            <v-divider class="mt-5" />
            <v-card-actions>
              <v-spacer />
              <v-btn text @click="$refs['config_form'].reset()">重置</v-btn>
              <v-btn
                tile
                color="primary"
                :loading="save_loading"
                @click="handleSubmitForm"
                >提交</v-btn
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

// now we can call our Command!
// Right-click the application background and open the developer tools.
// You will see "Hello, World!" printed in the console!
invoke("greet", { name: "World" })
  // `invoke` returns a Promise
  .then((response) => console.log(response));
export default {
  name: "NotionRss",
  components: {},
  props: {},
  data() {
    return {
      show: false,
      valid: true,
      save_loading: false,
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
    this.init_user();
    // date: (new Date(Date.now() - (new Date()).getTimezoneOffset() * 60000)).toISOString().substr(0, 10),
  },
  methods: {
    async init_user() {},
    handleSubmitForm() {
      if (this.$refs["config_form"].validate()) {
        this.btn_loading = true;
      }
    },
  },
};
</script>
