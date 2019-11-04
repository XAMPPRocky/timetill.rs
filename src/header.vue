<template>
    <header class="row sticky-top align-items-center">
        <nav class="col-12 text-center">
            <ul class="list-unstyled list-inline d-flex justify-content-around">
                <li class="list-inline-item">
                    <router-link to="/"><u>Conferences</u></router-link>
                </li>
                <li class="list-inline-item">
                    <router-link to="/events"><u>Events</u></router-link>
                </li>
                <li class="list-inline-item" v-if="user">
                    <router-link to="/profile" class="d-inline-block" style="vertical-align: middle">
                        <inline-user :user="user"></inline-user>
                        <span style="line-height: 0.1">▼</span>
                    </router-link>
                </li>
                <li class="list-inline-item" v-else>
                    <a :href="loginLink()"><u>Login with GitHub</u></a>
                </li>
            </ul>
        </nav>
        <div class="content col-12">
            <div class="row">
                <div class="col text-center">
                    <h1>Time till...</h1>
                </div>
                <div class="col d-none d-md-block">
                    <h1 class="conf-name text-center">
                        <a :href="event.website" target="_blank">
                            {{event.name}}
                        </a>
                    </h1>
                </div>
                <div class="col d-none d-md-block">
                    <time-slice class="text-center" :event="event"></time-slice>
                </div>
            </div>
        </div>
    </header>
</template>

<script lang="ts">
export default {
  methods: {
    loginLink () {
      return `http://localhost:5000/login?next_page=${window.location.pathname}`
    },
  }

  props: {
    event: Object,
    user: Object
  }
}
</script>

<style lang="scss" scoped>
header {
    position: -webkit-sticky;
    position: sticky;
    margin: 2rem 0;
    padding-top: 1rem;
    background-color: white;

    a {
        color: inherit;
        text-decoration: none;
    }
}

.content {
    border-bottom: 1px solid black;
    border-top: 1px solid black;
    box-shadow: 0px 10px 17px -11px rgba(0,0,0,0.25);
    padding: 1rem 0;

    a {
        text-decoration: underline;
    }
}
</style>
