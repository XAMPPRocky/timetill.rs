<template>
    <header class="row sticky-top align-items-center">
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
    // position: sticky;
    position: -webkit-sticky;
    margin: 2rem 0;
    padding: 1rem 0;
    background-color: white;

    border-top: 1px solid black;
    border-bottom: 1px solid black;

    a {
        color: inherit;
        text-decoration: underline;
    }
}
</style>
