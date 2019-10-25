import Router from 'vue-router'
import Vue from 'vue'
import moment from 'moment'

import App from './app.vue'
import Events from './events.vue'
import Home from './Home.vue'

import TimeSlice from './time-slice.vue'

// Conference data.
import data from '../conferences.json'
const conferences = data.filter(c => new Date(c.date) > new Date())
  // Sort by closest to furtherest from today.
  .sort((a, b) => new Date(a.date) - new Date(b.date))

const routes = [
  {
    path: '/',
    component: App,
    props: { conference: conferences[0] },
    children: [
      {
        path: '',
        component: Home,
        props: { conferences: conferences }
      },
      {
        path: 'events',
        component: Events
      }
    ]
  }
]

const router = new Router({ routes: routes })
Vue.use(Router)

Vue.component('time-slice', TimeSlice)
Vue.filter('date', (date) => moment(date).format('dddd, Do of MMMM YYYY'))

/* eslint-disable no-new */
new Vue({
  el: '#app',
  router
})
