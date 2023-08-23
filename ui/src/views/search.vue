<template>
    <navigation />
    <div class="container">
        <div class="row">
            <div class="col-md-6 offset-md-3">
                <div class="row g-4">
                    <h1>Search</h1>
                </div>
                <div class="row g-4">
                    <div class="input-group mb-2">
                        <input type="text" @keydown="clearNoMatch" v-model="q" placeholder="search customer name" class="form-control" maxlength="20" />
                    </div>
                </div>
                <div class="row g-4">
                    <div class="col-12">
                        <button class="btn btn-dark float-end" @click="searchCustomer">
                            <span v-if="!isLoading"><i class="bi-search"></i> &nbsp; search</span>
                                <span v-else>
                                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                                    <span class="sr-only">&nbsp; searching...</span>
                                </span>
                        </button>
                    </div>
                </div>    
                <div class="row mt-1">
                    <table class="table">
                        <tbody>
                            <tr v-for="customer in customers" key="customer.id">
                                <td>{{ customer.name }} ({{ customer.phone  }})</td>
                                <td><a :href="`/transactions/${customer.id}`"><span class="bi-cash-coin" /> Transactions</a> </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div class="row g-4 mt-1" v-if="noMatch">
                    <div class="col-12">
                        <p>
                            <span class="bi-exclamation-diamond"/> no matches found for <i>"{{ q }}"</i>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import Nav from '../components/nav.vue'
import Api from '../lib/api';

export default {
    data: () => ({
        q: '',
        customers: [],
        isLoading: false,
        noMatch: false
    }),
    mounted () {
        document.title = 'Baqi | Search'
        Api.isLoggedIn();
    },
    methods: {
        searchCustomer () {
            this.isLoading = true;
            this.noMatch = false;
            Api.searchCustomers(this.q)
                .then((data) => {
                    this.customers = data.data
                    this.isLoading = false
                    this.noMatch = (this.customers.length === 0)
                })
                .catch(({ response }) => {
                    console.log(JSON.stringify(response))
                    this.customers = []
                    this.isLoading = false
                })
        },
        clearNoMatch () {
            this.noMatch = false
        }
    },
    components: {
        Navigation: Nav
    }
}


</script>