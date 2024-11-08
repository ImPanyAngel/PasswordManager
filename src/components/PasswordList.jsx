import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Password from './Password';
import './components.css';

function PasswordList({ accountID, listKey }) {
    const [passwords, setPasswords] = useState([]);

    const fetchPasswords = async () => {
        try {
            const result = await invoke('get_account_passwords', { accountId: accountID });
            setPasswords(result);
        } catch (error) {
            console.error("Failed to fetch passwords:", error);
        }
    };

    useEffect(() => {
        fetchPasswords();
    }, [listKey]);

    return (
        <div className="password-list-container">
            <ul className='password-list'>
                {passwords.map(({ 0: passID, 1: website, 2: password }, index) => (
                    <li key={index}>
                        <strong>{website}:</strong>
                        <Password passwordID={passID} plainText={password} refreshPasswords={fetchPasswords} />
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default PasswordList;
