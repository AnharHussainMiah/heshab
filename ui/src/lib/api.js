import axios from "axios";
import Cookies from '../lib/cookies'
import Router from '../router';

const http = axios.create({});

http.interceptors.response.use((res) => {
    if (res.status == 403) {
      Cookies.deleteCookie("authToken");
      Router.push({ path: "/" });
      return res;
    }
    return res;
  });

export default {
    getAuthorizationHeader() {
        const authToken = Cookies.getCookie('authToken');
        return { Authorization: `${authToken}`, "Content-Type": "application/json" };
    },

    async searchCustomers(query) {
        const res = await http.post(`/api/search-customers/${encodeURI(query)}`, { }, { headers: this.getAuthorizationHeader()});
        return res
    },

    async getCustomerDetails(customerId) {
        const res = await http.post(`/api/get-customer-detail/${customerId}`, { }, { headers: this.getAuthorizationHeader()});
        return res
    },

    async getCustomerTransactions(customerId) {
        const res = await http.post(`/api/get-customer-transactions/${customerId}`, { }, { headers: this.getAuthorizationHeader()});
        return res
    },

    async addCustomerTransaction(payload) {
        const res = await http.post(`/api/add-customer-transaction/`, payload, { headers: this.getAuthorizationHeader()});
        return res
    },

    async deleteCustomerTransaction(transactionId) {
        const res = await http.post(`/api/delete-customer-transaction/${transactionId}`, { }, { headers: this.getAuthorizationHeader()});
        return res
    },

    async addNewCustomer(payload) {
        const res = await http.post(`/api/add-new-customer/`, payload, { headers: this.getAuthorizationHeader()});
        return res
    },

    async updateCustomer(payload) {
        const res = await http.post(`/api/update-customer/`, payload, { headers: this.getAuthorizationHeader()});
        return res
    },

    async isLoggedIn(isLoginPage) {
        http.post(`/api/confirm-is-logged-in`, { }, { headers: this.getAuthorizationHeader()})
            .then((data) => {
                console.log(data.data)
                if(isLoginPage) {
                    Router.push({ path: 'search'})
                }
            })
            .catch(({ response }) => {
                console.log(JSON.stringify(response.data));
                Router.push({ path: '/'})
            })
    },

    async login(payload) {
        const res = await http.post("/api/auth", payload, { headers: this.getAuthorizationHeader()});
        return res
    },

    saveAuthToken(token) {
        Cookies.setCookie('authToken', token); 
    },

    logOut() {
        Cookies.deleteCookie('authToken');
    }
}