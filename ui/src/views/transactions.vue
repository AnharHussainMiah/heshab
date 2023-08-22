<template>
    <navigation />
    <div class="container">
        <div class="row">
            <div class="col-md-6 offset-md-3">
                <div class="row g-4">
                    <h1>Transactions: Total (£{{  total }})</h1>
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
                        <a :href="`/customer/${customerId}`" class="btn btn-dark"><span class="bi-pencil" /> Edit Customer</a>
                    </div>
                </div>
                <div class="row g-4 mt-2 mb-4">
                    <div class="col-auto">
                        <button class="btn btn-dark" @click="btnShowPaymentModal"><span class="bi-bag-plus-fill" /> Add Payment</button>
                    </div>
                    <div class="col-auto">
                        <button class="btn btn-dark" @click="btnShowPaymentModal"><span class="bi-bag-dash-fill" /> Add Credit</button>
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
                                <td>£{{ transaction.amount  }}</td>
                                <td>{{ transaction.date_added  }}</td>
                                <td><button class="btn btn-light" @click="btnDeleteTransaction(transaction.id)"><span class="bi-trash-fill" /></button></td>
                            </tr>
                        </tbody>
                    </table>
                </div>

            
                    

                    <dialog id="paymentModal">
                        <h2>Add Transaction</h2>
                        <input type="number" class="form-control" v-model="amount">
                        <button class="btn btn-dark" @click="btnCancel">cancel</button>
                        <button class="btn btn-success" @click="btnSubmitTransaction">submit</button>
                    </dialog>

                    

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
        amount: 0.0,
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
        btnSubmitTransaction () {
            const payload = {
                customer_id: parseInt(this.$route.params.customerId),
                amount: this.amount
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
        }
    }
}
</script>