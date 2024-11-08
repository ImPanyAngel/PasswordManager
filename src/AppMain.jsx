import { invoke } from '@tauri-apps/api/core';
import { useState, useEffect } from 'react';
import Account from './components/Account';
import './main.css';

import addIcon from './assets/add-button.png';
import closeIcon from './assets/close-icon.png';

function AppMain() {
    const [isPopupVisible, setIsPopupVisible] = useState(false);
    const [accountName, setAccountName] = useState('');
    const [fetchData, setFetchData] = useState(true);
    const [users, setUsers] = useState([]);

    useEffect(() => {
        const getData = async () => {
            try {
                const userData = await invoke('get_user_data');
                setUsers(userData);
                setFetchData(false);
            } catch (error) {
                console.error("Failed to fetch user data:", error);
            }
        };

        if (fetchData) {
            getData();
        }
    }, [fetchData]);

    const togglePopup = () => {
        setIsPopupVisible(!isPopupVisible);
        setAccountName('');
    };

    const createAccount = async () => {
        try {
            if (accountName.trim() === "") {
                await invoke('insert_user_with_custom_id', { emailUsername: "Blank Field" });
            } else {
                await invoke('insert_user_with_custom_id', { emailUsername: accountName });
            }
            setFetchData(true);
            togglePopup();
        } catch (error) {
            console.error("Failed to create account:", error);
        }
    };

    return(
        <div className="accounts-container">

            {users.map((user, index) => {
                const [id, emailUsername] = user;
                return <Account key={index} emailUsername={emailUsername} id={id} onDelete={() => setFetchData(true)}/>
            })}

            {isPopupVisible && (
                <div className="popup-overlay">
                    <div className="popup">
                        <img className='close-popup' onClick={togglePopup} src={closeIcon} alt='close'/>
                        <p>Add a new account</p>
                        <input className="popup-input" autoFocus type='text' placeholder='Enter account username or email' value={accountName} onChange={(e) => setAccountName(e.target.value)}/>
                        <button className="confirm-popup-btn" onClick={createAccount}>Confirm</button>
                    </div>
                </div>
            )}

            <button className="add-account-btn" onClick={togglePopup}><img className="btn-add-img" src={addIcon} alt="add"/></button>
        </div>
    );
}

export default AppMain;
