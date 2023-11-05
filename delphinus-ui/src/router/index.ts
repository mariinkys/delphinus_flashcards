import { createRouter, createWebHistory } from 'vue-router'
import FaqView from '../views/FaqView.vue'
import HomeView from '../views/HomeView.vue'
import ResultsView from '../views/ResultsView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/faq',
      name: 'faq',
      component: FaqView
    },
    {
      path: '/results',
      name: 'results',
      component: ResultsView
    }
  ]
})

export default router
