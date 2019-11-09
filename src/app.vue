<template>
  <div>
    <div class="container">
      <time-header :event="conference"></time-header>
      <router-view></router-view>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { Component, Prop, Watch } from "vue-property-decorator";
import TimeHeader from "./header.vue";
import Consts from "./consts.ts";

@Component({ components: { TimeHeader } })
export default class App extends Vue {
  @Prop() conference: object;
  @Prop() user: object | null;
}
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

p a {
  color: inherit;
  font-weight: 600;
  text-decoration: underline;
}
</style>
