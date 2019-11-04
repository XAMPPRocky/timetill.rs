import Router from 'vue-router'
import Vue from 'vue'
import moment from 'moment'
import { Icon } from 'leaflet'

import App from './app.vue'
import EventDetail from './EventDetail.vue'
import EventList from './EventList.vue'
import Home from './home.vue'
import InlineUser from './InlineUser.vue'
import Consts from './consts.ts'

import TimeSlice from './time-slice.vue'

// Conference data.
import ConferenceData from './ConferenceData.ts'
const conferences = ConferenceData.getInstance().data()

// Leaflet integration
delete Icon.Default.prototype._getIconUrl
Icon.Default.mergeOptions({
  iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
  iconUrl: require('leaflet/dist/images/marker-icon.png'),
  shadowUrl: require('leaflet/dist/images/marker-shadow.png')
})

const routes = [
  {
    path: '/',
    component: App,
    props: { conference: conferences[0] },
    children: [
      {
        path: '',
        component: Home,
        name: 'home',
        props: { conferences }
      },
      {
        path: 'authorise',
        redirect (to) {
          window.localStorage.setItem(Consts.ACCESS_TOKEN_KEY, to.query.access_token)
          return { path: to.query.next_page, query: null }
        }
      },
      {
        path: 'events/:slug',
        component: EventDetail,
        name: 'eventDetail',
        props: true
      },
      {
        path: 'events',
        name: 'eventList',
        component: EventList
      }
    ]
  }
]

const router = new Router({ routes, mode: 'history' })
Vue.use(Router)

Vue.component('time-slice', TimeSlice)
Vue.component('inline-user', InlineUser)
Vue.filter('date', (date) => moment(date).format('dddd, Do of MMMM YYYY'))

/* eslint-disable no-new */
new Vue({
  el: '#app',
  router
})
