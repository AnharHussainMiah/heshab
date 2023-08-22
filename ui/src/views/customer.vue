<template>
    <navigation />
    <div class="container">
        <div class="row">
            <div class="col-md-6 offset-md-3">
                <div class="row g-4 mt-2">
                    <a v-if="customerId > 0" :href="`/transactions/${customerId}`"><span class="bi-arrow-left" /> back to transaction</a>
                    <h1>Customer</h1>
                </div>
                <div class="input-group mb-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text"><span class="bi-person-circle" /></span>
                    </div>
                    <input class="form-control" placeholder="enter name" type="text" v-model="customer.name" />
                </div>
                <div class="input-group mb-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text"><span class="bi-phone" /></span>
                    </div>
                    <input class="form-control" placeholder="enter phone number" type="text" v-model="customer.phone" />
                </div>
                <div class="input-group mb-2">
                    <div class="input-group-prepend">
                        <span class="input-group-text"><span class="bi-map" /></span>
                    </div>
                    <input class="form-control" placeholder="enter address" type="text" v-model="customer.address" />
                </div>
                <div class="row g-4 mt-1">
                    <div class="col-12">
                        <button class="btn btn-dark float-end" @click="btnSave"><span class="bi-save-fill" /> Save</button>
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
        customer: {}
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
                    alert("Customer Details saved!");
                })
                .catch(({ response }) => {
                    alert(`Unable to save customer -> ${JSON.stringify(response)}`);
                })
        },
        createNewCustomer () {

        }
    },
    components: {
        Navigation: Nav
    }
}

</script>