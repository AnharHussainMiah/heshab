<template>
    <div class="container">
        <div class="row">
            <div class="col-md-6 offset-md-3">
                <div class="login-form bg-light mt-4 p-4 border border-1 rounded-3">
                    <div class="text-center">
                        <img src="/assets/img/bg.jpg" class="img-fluid rounded"/>
                    </div>
                    <div class="row g-1">
                        <h1>Baqi</h1>
                        <div class="input-group mb-2">
                            <div class="input-group-prepend">
                                <span class="input-group-text"><span class="bi-envelope" /></span>
                            </div>
                            <input type="text" placeholder="enter email" class="form-control" v-model="email" />
                        </div>
                        <div class="input-group mb-2">
                            <div class="input-group-prepend">
                                <span class="input-group-text"><span class="bi-key" /></span>
                            </div>
                            <input type="password" placeholder="enter password" class="form-control" v-model="password" />
                        </div>
                        <div class="col-12">
                            <button class="btn btn-dark float-end" @click="btnSubmit">
                                <span v-if="!isLoading"><i class="bi-lock-fill"></i> &nbsp; login</span>
                                <span v-else>
                                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                                    <span class="sr-only">&nbsp; authenticating...</span>
                                </span>
                            </button>
                        </div>
                    </div>                
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import Api from '../lib/api';
import Router from '../router';

export default {
    name: 'Login',
    data: () => ({
        email: "",
        password: "",
        isLoading: false
    }),
    mounted () {
        document.title = 'Baqi | Sign In'
        Api.isLoggedIn(true);
    },
    methods: {
        btnSubmit () {
            this.isLoading = true;
            Api.login({ email: this.email, password: this.password })
                .then((data) => {
                    Api.saveAuthToken(data.data.token);
                    console.log(`==> authentication token has been presisted`)
                    Router.push({ path: '/search'})
                })
                .catch(({ response }) => {
                    console.log(JSON.stringify(response.data));
                    this.isLoading = false;
                })
        }
    }
}
</script>