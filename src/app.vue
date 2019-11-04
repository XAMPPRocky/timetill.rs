<template>
  <div>
    <div class="container">
      <time-header :event="conference" :user="user"></time-header>
      <router-view></router-view>
    </div>
  </div>
</template>

<script lang="ts">
import TimeHeader from "./header.vue";
import Consts from "./consts.ts";

export default {
  components: {
    TimeHeader
  },
  props: {
    conference: Object
  },
  data() {
    const token = localStorage.getItem(Consts.ACCESS_TOKEN_KEY);
    let user = localStorage.getItem(Consts.USER_KEY);
    const userETag = localStorage.getItem(Consts.USER_ETAG_KEY);

    if (user) {
      user = JSON.parse(user);
    }
    if (token) {
      fetch("https://api.github.com/user", {
        headers: new Headers({
          Authorization: `token ${token}`,
          "If-None-Match": userETag
        })
      })
        .then(r => {
          if (r.status === 304) {
            return this.user;
          } else {
            console.log(r.headers);
            localStorage.setItem(Consts.USER_ETAG_KEY, r.headers.get("ETag"));

            return r.json();
          }
        })
        .then(user => {
          if (user !== this.user) {
            localStorage.setItem(Consts.USER_KEY, JSON.stringify(user));
            this.user = user;
          }
        });
    }

    return {
      user
    };
  }
};
</script>

<style lang="scss">
.about {
  margin-bottom: 2rem;
}

.btn-github {
  background-color: #24292e;
  color: white;
}

.h-ruler {
  margin-top: 2rem;
  border-bottom: 1px solid black;
}
</style>
