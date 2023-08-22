<template>
    <nav class="navbar navbar-expand-lg navbar-light bg-light">
        <a class="navbar-brand" href="/search">&nbsp;<span class="bi-search" /> Search</a>
        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNav">
            <ul class="navbar-nav">
                <li class="nav-item active">
                    <a class="nav-link" href="/customer/-1"><span class="bi-person-fill-add" /> New Customer</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link active" href="#" data-bs-toggle="modal" data-bs-target="#staticBackdrop"><span class="bi-unlock" /> Log out</a>
                </li>
            </ul>
        </div>
    </nav>
    <!-- Modal -->
    <div class="modal fade" id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
        <div class="modal-header">
            <h5 class="modal-title" id="staticBackdropLabel">Confirmation</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>
        <div class="modal-body">
            Are you sure you want to log out?
        </div>
        <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal"><span class="bi-x-circle" /> Cancel</button>
            <button @click="btnLogout" type="button" class="btn btn-danger" data-bs-dismiss="modal"><span class="bi-box-arrow-right" /> Yes Logout</button>
        </div>
        </div>
    </div>
    </div>

</template>

<script>
import Api from '../lib/api';
import Router from '../router';

export default {
    methods: {
        btnLogout () {
            Api.logOut()
            Router.push({ path: '/'})
        }
    }
}
</script>