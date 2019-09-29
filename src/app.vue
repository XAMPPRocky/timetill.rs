<template>
    <div>
        <div class="container">
            <time-header :event="nextConference"></time-header>

            <section class="row">
                <div class="col-12 col-md-6 order-last order-md-first text-right about">
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

                    <a href="//github.com/XAMPPRocky/timetill.rs"
                       class="btn btn-github">GitHub</a>
                </div>

                <conference v-for="conference in conferences"
                            :conference="conference"></conference>
            </section>
        </div>
    </div>
</template>

<script lang="ts">
import timeHeader from './header.vue';
import Conference from './conference.vue';
import data from '../conferences.json';
import moment from 'moment';

const conferences = data.filter(c => new Date(c.date) > new Date())
                      .sort((a, b) => new Date(a.date) - new Date(b.date))

export default {
    components: {
        Conference,
        timeHeader,
    }
    computed: {
        nextConference: function() {
            return this.conferences[0]
        }
    }
    data() {
        return {
            conferences: conferences
        }
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
</style>
