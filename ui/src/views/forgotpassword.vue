<template>
    <div class="container">
        <div class="row">
            <div class="col-md-6 offset-md-3">
                <div class="login-form bg-light mt-4 p-4 border border-1 rounded-3">
                    <div v-if="panel==='request'" class="row g-1">
                        <h1>Reset Password</h1>
                        <div class="input-group mb-2">
                            <div class="input-group-prepend">
                                <span class="input-group-text"><span class="bi-envelope" /></span>
                            </div>
                            <input type="text" placeholder="enter email" class="form-control" v-model="email" maxlength="100" />
                        </div>
                        <div class="col-12">
                            <button class="btn btn-dark float-end" @click="btnSubmit">
                                <span v-if="!isLoading"><i class="bi-lock-fill"></i> &nbsp; reset</span>
                                <span v-else>
                                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                                    <span class="sr-only">&nbsp; requesting...</span>
                                </span>
                            </button>
                        </div>
                        <div class="col-12 mt-3">
                            <div class="alert alert-warning" role="alert" v-if="errorMessage !== ''">
                                {{ errorMessage }}
                            </div>
                        </div>
                    </div>
                    <div v-if="panel==='completed'" class="row g-1">
                        <h1><span class="bi-check2-circle" /> Reset Requested</h1>
                        <p class="text-muted">
                            Your password reset has been requested. Please contact the admin team who will provide you with the reset link. The request is valid for only 24 hours.
                            If we have not received your request it will be because the email you have provided does not match any of our records.
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
        isLoading: false,
        errorMessage: "",
        panel: "request"
    }),
    mounted () {
        document.title = 'Baqi | Oops I forgot my password!'
    },
    methods: {
        btnSubmit () {
            this.isLoading = true;
            this.errorMessage = "";
            const payload = {
                email: this.email,
                token: "",
                password: ""
            };

            Api.resetPasswordRequest(payload)
                .then((data) => {
                    console.log(JSON.stringify(data.data))
                    this.panel = "completed";
                })
                .catch(({ response }) => {
                    console.log(JSON.stringify(response.data));
                    this.isLoading = false;
                    if(response.status === 401) {
                        this.errorMessage = "email or password is not valid!"
                    }
                })
        }
    }
}
</script>