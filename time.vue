<template>
    <div class="time row text-center">
        <div class="col">
        <template v-for="slice in slices">
            <div class="d-inline-block">
                <h1 class="d-md-none">{{slice.time}}</h1>
                <h2 class="d-none d-md-block">{{slice.time}}</h2>
                <h6 class="d-inline text-uppercase">{{slice.period}}</h6>
            </div>
            <h1 class="divider align-top">:</h1>
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
        generateSlices: function () {
            const today = moment()
            const confDate = moment(this.time)
            const duration = moment.duration(confDate.diff(today))
            let slices = []

            if (duration.asDays() !== 0) {
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
    line-height: 0.85;
    display: inline-block;

    &:last-of-type {
        display: none;
    }
}
</style>
