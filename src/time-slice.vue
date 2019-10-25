<template>
    <div class="time row">
        <div class="col">
        <template v-for="slice in slices">
            <div class="d-inline-block" :class="over100Days">
                <h1 class="d-md-none text-monospace">{{slice.time}}</h1>
                <h2 class="d-none d-md-block text-monospace">{{slice.time}}</h2>
                <h6 class="d-inline text-uppercase">{{slice.period}}</h6>
            </div>
            <h1 class="divider align-top" :class="over100Days">:</h1>
        </template>
        </div>
    </div>
</template>

<script lang="ts">
import moment from 'moment';

export default {
    data() {
        return {
            slices: []
        }
    }
    computed: {
        over100Days() {
            return { 'small-numbers': this.getDuration().asDays() >= 100 }
        }
    }
    created() {
        this.generateSlices()
        window.setInterval(this.generateSlices, 1000)
    }
    destroyed() {
        clearInterval(this.generateSlices)
    }
    props: {
        time: String
    }
    methods: {
        getDuration: function () {
            const today = moment()
            const confDate = moment(this.time)
            return moment.duration(confDate.diff(today))
        }
        generateSlices: function () {
            const duration = this.getDuration()
            let slices = []

            if (duration.asDays() >= 2) {
                slices.push({
                    time: Math.floor(duration.asDays()).toString().padStart(2, "0"),
                    period: "days"
                })
            } else {
                slices.push({
                    time: duration.hours().toString().padStart(2, "0"),
                    period: "hrs"
                })
            }

            slices.push({
                time: duration.minutes().toString().padStart(2, "0"),
                period: "mins"
            })

            slices.push({
                time: duration.seconds().toString().padStart(2, "0"),
                period: "secs"
            })

            this.slices = slices
        }
    }
}
</script>

<style lang="scss" scoped>
.time {
    h2 {
        margin-bottom: 0;
    }
}

.divider {
    line-height: 0.69;
    display: inline-block;

    &:last-of-type {
        display: none;
    }
}

.small-numbers {
    h1 {
        font-size: 2.1rem;
    }
    h2 {
        font-size: 1.6rem;
    }

    &.divider {
        line-height: 0.5;
    }
}
</style>
