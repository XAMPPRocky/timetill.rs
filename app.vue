<template>
    <div>
        <div class="container">
            <header class="row sticky-top align-items-center">
                <div class="col text-center">
                    <h1>Time till...</h1>
                </div>
                <div class="col d-none d-md-block">
                    <h1 class="conf-name text-center">
                        <a :href="nextConference.website" target="_blank">
                            {{nextConference.name}}
                        </a>
                    </h1>
                </div>
                <div class="col d-none d-md-block">
                    <time-slice :time="nextConference.date"></time-slice>
                </div>
            </header>
            <section class="row">
                <div class="col-12 col-md-6 order-last order-md-first text-right about">
                    <div class="text-left">
                        <h2>About timetill.rs</h2>
                        <p>
                        <a href="/">timetill.rs</a> is a community project focused
                        on highlighting all the Rust conferences around the world.
                        Timetill.rs is an open project that anyone in the community
                        can contribute to.
                        </p>
                        <p>Want your conference featured? We have documentation on
                        adding events in our <a href="#">how to contribute document.</a></p>
                    </div>

                    <a href="#" class="btn btn-github">GitHub</a>
                </div>

                <div v-for="conference in conferences" class="col-12 col-md-6 conference">
                    <div class="row h-100">
                        <div class="col-12 col-sm-5 col-md-6">
                            <h2>{{conference.name}}</h2>
                            <h6 class="d-none d-md-block">{{when(conference.date)}}, {{conference.location}}</h6>
                        </div>
                        <div class="col-12 col-sm-7 col-md-6 date">
                            <h6 class=" text-center d-md-none">{{when(conference.date)}}<br>{{conference.location}}</h6>
                            <time-slice :time="conference.date"></time-slice>
                        </div>
                        <div class="col-12 flex-grow-1">
                            <div class="row">
                                <div class="col-12 blurb">
                                    <p><em>&ldquo;{{conference.blurb}}</em>&rdquo;</p>
                                    <p class="cfp text-info" v-if="conference.cfp && isInFuture(conference.cfp.end)">
                                    This conference is looking for speakers until
                                    <a :href="conference.cfp.link">{{when(conference.cfp.end)}}</a>.
                                    </p>
                                </div>
                                <div class="col-12">
                                    <dl class="row">
                                        <template v-for="(feature, name) in conference.features">
                                            <dt class="col-6 text-capitalize">{{name}}</dt>
                                            <dd class="col-6 text-capitalize text-right">{{feature}}</dd>
                                        </template>
                                    </dl>
                                </div>
                            </div>
                        </div>
                        <div class="col-12 text-right call-to-action align-self-end">
                            <div class="row">
                                <div class="col-12">
                                    <a :href="conference.schedule" target="_blank" class="text-uppercase btn btn-secondary">schedule</a>
                                    <a :href="conference.website" target="_blank" class="text-uppercase btn btn-primary">website</a>
                                    <div class="w-100 divider"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    </div>
</template>

<script lang="ts">
import Vue from "vue";
import timeSlice from './time.vue';
import data from './data.json';
import moment from 'moment';

let conferences = data.filter(c => new Date(c.date) > new Date())

export default Vue.extend({
    components: {
        timeSlice
    }
    computed: {
        nextConference: function() {
            return this.conferences[0]
        }
    }
    methods: {
        when: function (time) {
            let momentDate = moment(time)

            return momentDate.format("dddd, Do of MMMM YYYY")
        },
        isInFuture: function(time) {
            let momentDate = moment(time)

            return momentDate > moment()
        }
    }
    data() {
        return {
            conferences: conferences
        }
    }
});
</script>

<style lang="scss" scoped>
header {
    // position: sticky;
    position: -webkit-sticky;
    margin: 2rem 0;
    padding: 2rem 0;
    background-color: white;

    border-top: 1px solid black;
    border-bottom: 1px solid black;

    a {
        color: inherit;
        text-decoration: underline;
    }
}

.about {
    margin-bottom: 2rem;
}

.conference {
    margin-bottom: 2rem;
    h2 {
        margin-bottom: 0;
    }

    .divider {
        margin-top: 2rem;
        border-bottom: 1px solid black;
    }
}

.blurb {
    margin-top: 2rem;
    margin-bottom: 1rem;
}

.btn-github {
    background-color: #24292e;
    color: white;
}

.call-to-action a {
    // font-size: 80%;
    font-weight: 500;
}

.cfp {
    a, a:hover {
        text-decoration: underline;
        font-weight: 600;
        color: inherit;
    }
}

@media (max-width: 767px) {
    .date {
        margin-top: 2rem;
    }

}

@media (min-width: 768px) {
    section > div {
        padding: 0 4rem;
    }
}

</style>
