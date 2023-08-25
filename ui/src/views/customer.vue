<template>
    <navigation />
    <div class="container">
        <div class="row">
            <div class="col-md-6 offset-md-3">
                <div class="row g-4 mt-2">
                    <router-link v-if="customerId > 0" :to="{ path: `/transactions/${customerId}`}"><span class="bi-arrow-left" /> back to transaction</router-link>
                    <h1>Customer</h1>
                </div>
                <div class="input-group mb-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text"><span class="bi-person-circle" /></span>
                    </div>
                    <input class="form-control" placeholder="enter name" type="text" v-model="customer.name" maxlength="50" />
                </div>
                <div class="input-group mb-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text"><span class="bi-phone" /></span>
                    </div>
                    <input class="form-control" placeholder="enter phone number" type="text" v-model="customer.phone" maxlength="11" />
                </div>
                <div class="input-group mb-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text"><span class="bi-map" /></span>
                    </div>
                    <input class="form-control" placeholder="enter address" type="text" v-model="customer.address" maxlength="100" />
                </div>
                <div v-if="successMessage===''" class="row g-4 mt-1">
                    <div class="col-12">
                        <button class="btn btn-dark float-end" @click="btnSave"><span class="bi-save-fill" /> Save</button>
                    </div>
                </div>
                <div class="row g-4 mt-1">
                    <div class="col-12">
                        <div class="alert alert-warning" role="alert" v-if="errorMessage !== ''">
                            <span class="bi-exclamation-diamond" /> {{ errorMessage }}
                        </div>
                    </div>
                </div>
                <div class="row g-4 mt-1">
                    <div class="col-12">
                        <div class="alert alert-success" role="alert" v-if="successMessage !== ''">
                            <span class="bi-check2-circle" /> {{ successMessage }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import Api from '../lib/api';
import Nav from '../components/nav.vue'

export default {
    data: () => ({
        customerId: -1,
        customer: {
            name: "",
            phone: "",
            address: ""
        },
        errorMessage: "",
        successMessage: ""
    }),
    mounted () {
        document.title = 'Baqi | Customer'
        Api.isLoggedIn();
        this.customerId = parseInt(this.$route.params.customerId);
        if(this.customerId > 0) {
            this.loadCustomerDetails();
        }
    },
    methods: {
        loadCustomerDetails () {
            Api.getCustomerDetails(this.customerId)
                .then((data) => {
                    this.customer = data.data;
                })
                .catch(({ response }) => {
                    console.log(JSON.stringify(response));
                });
        },
        btnSave () {
            this.successMessage = ""
            this.errorMessage = "";
            if(this.customerId > 0) {
                this.saveCustomer();
            } else {
                this.createNewCustomer();
            }
        },
        saveCustomer () {
            const payload = {
                customer_id: this.customerId,
                name: this.customer.name,
                phone: this.customer.phone,
                address: this.customer.address
            };
            Api.updateCustomer(payload)
                .then((data) => {
                    this.successMessage = "Customer details have been updated!";
                })
                .catch(({ response }) => {
                    if(response.status === 400) {
                        this.errorMessage = response.data
                    }
                    if(response.status === 500) {
                        this.errorMessage = "Sorry there was some error and we were not able to update the customer"
                    }
                })
        },
        createNewCustomer () {
            const payload = {
                customer_id: this.customerId,
                name: this.customer.name,
                phone: this.customer.phone,
                address: this.customer.address
            };
            Api.addNewCustomer(payload)
                .then((data) => {
                    this.successMessage = "New customer has been created!";
                })
                .catch(({ response }) => {
                    if(response.status === 400) {
                        this.errorMessage = response.data
                    }
                    if(response.status === 500) {
                        this.errorMessage = "Sorry there was some error and we were not able to update the customer"
                    }
                })
        }
    },
    components: {
        Navigation: Nav
    }
}

</script>