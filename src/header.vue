<template>
    <header class="row sticky-top align-items-center">
        <nav class="col-12 text-center">
            <ul class="list-unstyled list-inline">
                <li class="list-inline-item">
                    <router-link to="/">Conferences</router-link>
                </li>
                <li class="list-inline-item">
                    <router-link to="/events">Events</router-link>
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
                    <time-slice class="text-center" :time="event.date"></time-slice>
                </div>
            </div>
        </div>
    </header>
</template>

<script lang="ts">
export default {
  props: {
    event: Object
  }
}
</script>

<style lang="scss" scoped>
header {
    position: -webkit-sticky;
    position: sticky;
    margin: 2rem 0;
    padding: 1rem 0;
    background-color: white;

    a {
        color: inherit;
        text-decoration: underline;
    }
}

.content {
    border-top: 1px solid black;
    border-bottom: 1px solid black;
    padding: 1rem 0;
}
</style>
