import { createRouter, createWebHistory } from 'vue-router'

import Login from '../views/login.vue'
import Search from '../views/search.vue'
import Customer from '../views/customer.vue'
import Transactions from '../views/transactions.vue'
import ForgotPassword from '../views/forgotpassword.vue'
import NewPassword from '../views/newpassword.vue'


const routes = [
    {
        path: '/',
        name: 'Login',
        component: Login
    },
    {
        path: '/search',
        name: 'Search',
        component: Search
    },
    {
        path: '/transactions/:customerId',
        name: 'transactions',
        component: Transactions
    },
    {
        path: '/customer/:customerId',
        name: 'customer',
        component: Customer
    },
    {
        path: '/forgot-password',
        name: 'ForgotPassword',
        component: ForgotPassword
    },
    {
        path: '/new-password',
        name: 'NewPassword',
        component: NewPassword
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes
  })
  
export default router