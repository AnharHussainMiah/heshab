<template>
    <div class="container">
        <div class="row">
            <div class="column column-60 column-offset-25">
                <h3>Login:</h3>
                <input type="text" placeholder="📧 enter email" v-model="email" />
                <input type="password" placeholder="🔑 enter password" v-model="password" />
                <button class="button" @click="btnSubmit">🔒 login</button>
            </div>
        </div>
    </div>
</template>

<script>


export default {
    name: 'Login',
    data: () => ({
        email: "",
        password: ""
    }),
    mounted () {

    },
    methods: {
        btnSubmit () {
            
        }
    }
}
</script>