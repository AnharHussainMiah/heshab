<template>
    <navigation />
    <div class="container">
        <div class="row">
            <div class="col-md-6 offset-md-3">
                <div class="row g-4">
                    <h1>Transactions: Total (£{{  (total/100).toFixed(2) }})</h1>
                </div>
                <div class="card">
                    <div class="card-header">
                        Customer Details
                    </div>
                    <div class="card-body">
                        <h5 class="card-title">Name: {{ customer.name }}</h5>
                        <p class="card-text">
                        <span class="bi-phone" />{{ customer.phone  }}<br/>
                        <span class="bi-map"/> {{ customer.address }}
                        </p>
                        <router-link :to="{ path: `/customer/${customerId}`}" class="btn btn-dark"><span class="bi-pencil" /> Edit Customer</router-link>
                    </div>
                </div>
                <div class="row g-4 mt-2 mb-4">
                    <div class="col-auto">
                        <button class="btn btn-dark" data-bs-toggle="modal" data-bs-target="#transactionModal"><span class="bi-cash-coin" /> Add Payment</button>
                    </div>
                </div>
                <div class="row g-4">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>Amount</th>
                                <th>Date Added</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr v-for="transaction in transactions" key="transaction.id">
                                <td>£{{ (transaction.amount/100).toFixed(2)  }}</td>
                                <td>{{ formatDate(transaction.date_added)  }}</td>
                                <td><button class="btn btn-light" @click="btnDeleteTransaction(transaction.id)"><span class="bi-trash-fill" /></button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>

            
                    <!-- Modal -->
                    <div class="modal fade" id="transactionModal" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                        <div class="modal-dialog">
                            <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title">Add Transaction</h5>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body">
                                <input type="text" class="form-control" v-model="amount" maxlength="10"  >
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal"><span class="bi-x-circle" /> Cancel</button>
                                <button class="btn btn-success" @click="btnSubmitTransaction(false)" data-bs-dismiss="modal">Paid</button>
                                <button class="btn btn-warning" @click="btnSubmitTransaction(true)" data-bs-dismiss="modal">Baqi</button>
                            </div>
                            </div>
                        </div>
                    </div>
                    <!-- Modal -->
            </div>
        </div>
    </div>
</template>


<script>
import Api from '../lib/api';
import Nav from '../components/nav.vue'


export default {
    data: () => ({
        transactions: [],
        showPaymentModal: false,
        amount: "",
        modal: null,
        customer: {},
        customerId: -1
    }),
    components: {
        Navigation: Nav
    },
    mounted () {
        document.title = 'Baqi | Transactions'
        this.modal = document.getElementById("paymentModal");
        this.customerId = parseInt(this.$route.params.customerId);
        Api.isLoggedIn();
        this.loadCustomerDetails();
        this.loadTransactions();
    },
    computed: {
        total () {
            return this.transactions.map(x => x.amount).reduce((a,b) => a + b, 0);
        }
    },
    methods: {
        loadTransactions () {
            Api.getCustomerTransactions(this.customerId)
                .then((data) => {
                    this.transactions = data.data;
                })
                .catch(({ response }) => {

                })
        },
        loadCustomerDetails () {
            Api.getCustomerDetails(this.customerId)
                .then((data) => {
                    this.customer = data.data
                })
                .catch(({ response }) => {
                    console.log(JSON.stringify(response));
                })
        },
        btnShowPaymentModal() {
            if(this.modal) {
                this.modal.showModal();
            }
        },
        btnCancel () {
            this.modal.close();
        },
        btnSubmitTransaction (isBaqi) {
            const total = parseInt(this.amount * 100);

            const payload = {
                customer_id: parseInt(this.$route.params.customerId),
                amount: (isBaqi===true)? -total: total
            };

            Api.addCustomerTransaction(payload)
                .then((data) => {
                    this.amount = 0.0;
                    this.loadTransactions();
                    this.modal.close();
                })
                .catch(({ response }) => {
                    console.log(JSON.stringify(response));
                })
        },
        btnDeleteTransaction(id) {
            if(confirm("Are you sure you want to delete this transaction?")) {
                const transactionId = parseInt(id);
                Api.deleteCustomerTransaction(id)
                    .then((data) => {
                        this.loadTransactions();
                    })
                    .catch(({ response }) => {
                        console.log(JSON.stringify(response));
                    })
            }
        },
        formatDate (ds) {
            const f = new Date(ds);
            return f.toLocaleString();
        }
    }
}
</script>