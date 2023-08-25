<template>
    <div class="container">
        <div class="row">
            <div class="col-md-6 offset-md-3">
                <div class="login-form bg-light mt-4 p-4 border border-1 rounded-3">
                    <div v-if="panel==='update'" class="row g-1">
                        <h1>Set New Password</h1>
                        <div class="input-group mb-2">
                            <div class="input-group-prepend">
                                <span class="input-group-text"><span class="bi-envelope" /></span>
                            </div>
                            <input type="text" placeholder="enter email" class="form-control" v-model="email" maxlength="100" />
                        </div>
                        <div class="input-group mb-2">
                            <div class="input-group-prepend">
                                <span class="input-group-text"><span class="bi-key" /></span>
                            </div>
                            <input type="password" placeholder="enter password" class="form-control" v-model="password" maxlength="50" />
                        </div>
                        <div class="col-12">
                            <button class="btn btn-dark float-end" @click="btnSubmit">
                                <span v-if="!isLoading"><i class="bi-lock-fill"></i> &nbsp; update</span>
                                <span v-else>
                                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                                    <span class="sr-only">&nbsp; updating...</span>
                                </span>
                            </button>
                        </div>
                        <div class="col-12 mt-3">
                            <div class="alert alert-warning" role="alert" v-if="errorMessage !== ''">
                                <span class="bi-exclamation-diamond" /> {{ errorMessage }}
                            </div>
                        </div>
                    </div>
                    <div v-if="panel==='completed'" class="row g-1">
                        <h1><span class="bi-check2-circle" /> Updated!</h1>
                        <p class="text-muted">
                            Your new password has now been set, you should now be able to login using your new password.
                            To login now click <router-link :to="{ path: '/'}">here</router-link>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import Api from '../lib/api';

export default {
    name: 'Login',
    data: () => ({
        email: "",
        password: "",
        token: '',
        isLoading: false,
        errorMessage: "",
        panel: 'update'
    }),
    mounted () {
        document.title = 'Baqi | Set new password'
        const q = this.$route.query;
        if(q.token) {
            this.token = q.token;
        }
    },
    methods: {
        btnSubmit () {
            this.isLoading = true;
            this.errorMessage = "";
            const payload = {
                email: this.email,
                password: this.password,
                token: this.token
            };

            Api.updateNewPassword(payload)
                .then((data) => {
                    console.log(JSON.stringify(data.data))
                    this.panel = "completed";
                })
                .catch(({ response }) => {
                    console.log(JSON.stringify(response.data));
                    this.isLoading = false;
                    if(response.status === 400) {
                        this.errorMessage = response.data
                    }
                })
        }
    }
}
</script>