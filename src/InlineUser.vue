<template>
  <span v-if="linkGithub" class="d-inline-block">
    <a target="_blank" :href="user.html_url" class="d-inline-block">
      <img :src="user.avatar_url" height="24px" />
    </a>
    <a target="_blank" :href="user.html_url">
      {{ user.login }}
    </a>
  </span>
  <span v-else class="d-inline-block">
    <img :src="user.avatar_url" height="24px" />
    {{ user.login }}
  </span>
</template>

<script lang="ts">
export default {
  props: {
    linkGithub: Boolean,
    user: Object
  }
};
</script>

<style lang="scss" scoped>
span,
span > * {
  vertical-align: middle;
}

img {
  border-radius: 10%;
}

a {
  color: inherit;
  text-decoration: underline;
}
</style>
