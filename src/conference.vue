<template>
    <div class="col-12 col-md-6 conference">
        <div class="row h-100">
            <div class="col-12 col-sm-5 col-md-6">
                <h2>{{conference.name}}</h2>
                <h6 class="d-none d-md-block">
                    {{conference.date | date}}, {{conference.location}}
                </h6>
            </div>
            <div class="col-12 col-sm-7 col-md-6 date">
                <h6 class="text-center d-md-none">{{conference.date | date}}<br>{{conference.location}}</h6>
                <time-slice class="text-center text-md-right" :time="conference.date"></time-slice>
            </div>
            <div class="col-12 flex-grow-1">
                <div class="row">
                    <div class="col-12 blurb">
                        <p><em>&ldquo;{{conference.blurb}}&rdquo;</em></p>
                        <p class="cfp text-info"
                           v-if="conference.cfp && isInFuture(conference.cfp.end)">
                            This conference is looking for speakers until
                            <a :href="conference.cfp.link">{{conference.cfp.end | date}}</a>.
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
                        <a :href="conference.schedule"
                           target="_blank"
                           class="text-uppercase btn btn-secondary">schedule</a>
                        <a :href="conference.website"
                           target="_blank"
                           class="text-uppercase btn btn-primary">website</a>
                        <div class="w-100 h-ruler"></div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    </div>
</template>

<script lang ="ts">
import moment from 'moment';

export default {
    props: {
        conference: Object
    }
    methods: {
        isInFuture(time) {
            return moment(time) > moment()
        }
    }
}
</script>

<style lang="scss" scoped>
.blurb {
    margin-top: 2rem;
    margin-bottom: 1rem;
}

.conference {
    margin-bottom: 2rem;

    h2 {
        margin-bottom: 0;
    }

}

.cfp {
    a, a:hover {
        text-decoration: underline;
        font-weight: 600;
        color: inherit;
    }
}

.call-to-action a {
    // font-size: 80%;
    font-weight: 500;
}


@media (max-width: 767px) {
    .date {
        margin-top: 2rem;
    }
}

</style>
