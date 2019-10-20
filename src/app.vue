<template>
    <div>
        <div class="container">
            <time-header :event="conference"></time-header>
            <router-view></router-view>
        </div>
    </div>
</template>

<script lang="ts">
import TimeHeader from './header.vue'

export default {
  components: {
      TimeHeader,
  }
  props: {
    conference: Object
  }
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
</style>
