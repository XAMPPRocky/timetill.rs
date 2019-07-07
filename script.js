
import Vue from 'vue';
import App from './app.vue';

new Vue(App).$mount('#app')

/*
function main(current) {
    const confDate = new Date('4/27/2019 16:10:00')
    const heading = document.querySelector('#time')
    const diffTime = Math.abs(confDate.getTime() - current)

    const days = Math.floor(diffTime / (1000 * 60 * 60 * 24))
    const dayDiff = diffTime - (days * 24 * 60 * 60 * 1000)
    const hours = Math.abs(Math.floor(dayDiff / (1000 * 60 * 60)))
    const hourDiff = dayDiff - (hours * 60 * 60 * 1000)
    const minutes = Math.abs(Math.floor(hourDiff / (1000 * 60)))
    const minuteDiff = hourDiff - (minutes * 1000 * 60)
    const seconds = Math.abs(Math.floor(minuteDiff / (1000)))
    const secondDiff = minuteDiff - (seconds * 1000)

    const displayTime = `${format(days, 2)}:${format(hours, 2)}:${format(minutes, 2)}:${format(seconds, 2)}.${format(secondDiff, 3)}`
    heading.innerHTML = displayTime
    requestAnimationFrame(main)
}

requestAnimationFrame(main)

function format(num, size) {
    let s = num.toString()

    while (s.length < size) {
        s = "0" + s
    }

    return s
}
*/
