import Vue from 'vue';
import App from './app.vue';
import TimeSlice from './time-slice.vue';
import moment from 'moment';

Vue.component('time-slice', TimeSlice)

Vue.filter('date', (date) => moment(date).format("dddd, Do of MMMM YYYY"))

new Vue({
    el: '#app',
    render: h => h(App)
})
