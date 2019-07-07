<template>
    <div class="time row text-center">
        <div class="col">
        <template v-for="slice in slices">
            <div class="d-inline-block">
                <h2>{{slice.time}}</h2>
                <h6 class="d-inline text-uppercase">{{slice.period}}</h6>
            </div>
            <h1 class="divider d-inline-block">:</h1>
        </template>
        </div>
    </div>
</template>

<script lang="ts">
export default {
    data() {
        return {
            slices: []
        }
    }
    created() {
        window.setInterval(this.generateSlices, 100)
    }
    destroyed() {
        clearInterval(this.generateSlices)
    }
    props: {
        time: Date
    }
    methods: {
        generateSlices: function () {
            const today = new Date()
            const diffTime = Math.abs(this.time.getTime() - today)
            const days = Math.floor(diffTime / (1000 * 60 * 60 * 24))
            const dayDiff = diffTime - (days * 24 * 60 * 60 * 1000)
            const hours = Math.abs(Math.floor(dayDiff / (1000 * 60 * 60)))
            const hourDiff = dayDiff - (hours * 60 * 60 * 1000)
            const minutes = Math.abs(Math.floor(hourDiff / (1000 * 60)))
            const minuteDiff = hourDiff - (minutes * 1000 * 60)
            const seconds = Math.abs(Math.floor(minuteDiff / (1000)))
            const secondDiff = minuteDiff - (seconds * 1000)

            this.slices = [
                {
                    "time": days,
                    "period": "days"
                },
                {
                    "time": minutes,
                    "period": "mins"
                },
                {
                    "time": seconds.toString().padStart("0", 2),
                    "period": "secs"
                },
            ]

        }
    }
}
</script>

<style lang="scss" scoped>
.time .col .divider:last-child {
    display: none;
}
</style>
