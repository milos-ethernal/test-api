import React, { Component } from 'react';
import axios from 'axios';

class TransactionList extends Component {
    constructor(props) {
        super(props);
        this.state = {
            transactions: [],
            loading: true,
        };
    }

    componentDidMount() {
        // Replace with the actual URL where your API is hosted
        const apiUrl = 'http://localhost:9090/transactions';

        axios.get(apiUrl)
            .then((response) => {
                console.log(response);
                this.setState({
                    transactions: response.data,
                    loading: false,
                });
            })
            .catch((error) => {
                console.error('Error fetching data:', error);
                this.setState({ loading: false });
            });
    }

    render() {
        const { transactions, loading } = this.state;

        return (
            <div>
                <h1>Transaction List</h1>
                {loading ? (
                    <p>Loading...</p>
                ) : (
                    <table>
                        <thead>
                            <tr>
                                <th>Transaction ID</th>
                                <th>Result</th>
                                <th>Computation Start</th>
                                <th>Computation End</th>
                                <th>Is Initiator</th>
                            </tr>
                        </thead>
                        <tbody>
                            {transactions.map((transaction) => (
                                <tr key={transaction.transaction_id}>
                                    <td>{transaction.transaction_id}</td>
                                    <td>{transaction.result}</td>
                                    <td>{transaction.computation_start}</td>
                                    <td>{transaction.computation_end}</td>
                                    <td>{transaction.is_initiator ? 'Yes' : 'No'}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                )}
            </div>
        );
    }
}

export default TransactionList;
