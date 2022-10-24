<template>
    <div class="container">
        <section class="row">
            <div class="col-12 col-md-6 order-last order-md-first text-right about">
                <div class="row h-100">
                    <div class="col">
                        <div class="text-left">
                            <h2>About timetill.rs</h2>
                            <p>
                            <a href="/">timetill.rs</a> is a community project
                            focused on highlighting Rust conferences from around
                            the world.
                            Timetill.rs is an open project that anyone in the
                            community can contribute to.
                            </p>
                            <p>
                            Want your conference featured? We have documentation
                            on adding events in our
                            <a href="//github.com/XAMPPRocky/timetill.rs/blob/master/CONTRIBUTING.md">
                                how to contribute document.
                            </a>
                            </p>
                        </div>

                        <a href="//github.com/XAMPPRocky/timetill.rs/blob/master/CONTRIBUTING.md"
                           class="btn btn-primary">Add Your Event</a>
                        <a href="//github.com/XAMPPRocky/timetill.rs"
                           class="btn btn-github">View Source on GitHub</a>
                    </div>
                    <div class="col-12 align-self-end">
                        <div class="w-100 h-ruler"></div>
                    </div>
                </div>
            </div>

            <div v-if="conferences.length > 0">
            <conference v-for="conference in conferences" :key="conference.name"
                        :conference="conference"></conference>
            </div>
            <div v-else>
                There are no conferences currently listed. To add an event, check out
                <a href="//github.com/XAMPPRocky/timetill.rs/blob/master/CONTRIBUTING.md">the contributing guide</a>.
            </div>
        </section>
    </div>
</template>

<script lang="ts">
import Conference from './conference.vue'

export default {
  components: {
    Conference
  },
  props: {
    conferences: Array
  }
}
</script>
