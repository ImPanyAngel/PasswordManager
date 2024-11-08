import { invoke } from '@tauri-apps/api/core';
import { useState, useRef } from 'react';
import { Bounce, ToastContainer, toast } from 'react-toastify';
import PasswordList from './PasswordList';
import './components.css';

import 'react-toastify/dist/ReactToastify.css';

import upArrow from '../assets/arrow-up.png';
import downArrow from '../assets/arrow-down.png';
import deleteIcon from '../assets/delete-icon.png';
import addIcon from '../assets/add-button.png';
import eyeShow from '../assets/eye-show.png';
import eyeHide from '../assets/eye-hide.png';
import confirmIcon from '../assets/tick-icon.png';
import closeIcon from '../assets/close-icon.png';

function Account({emailUsername, id, onDelete}) {
    const [dropdown, setDropdown] = useState(false);
    const [passwordToggle, setPasswordToggle] = useState(false);
    const [toggleView, setToggleView] = useState("hide");
    const [passwordType, setPasswordType] = useState("password");
    const [passwordValue, setPasswordValue] = useState('');
    const [websiteName, setWebsiteName] = useState('');
    const [passwordListKey, setPasswordListKey] = useState(false);

    const clickTimeout = useRef(null);

    const notify = () => {
        toast("Double click to delete");
    };

    const handlePasswordClose = () => {
        setPasswordToggle(false);
        setToggleView("hide");
        setPasswordType("password");
        setPasswordValue('');
        setWebsiteName('');
    };

    const toggleDropdown = async () => {
        setDropdown(!dropdown);
        handlePasswordClose();
    };

    const handleToggleView = () => {
        if (toggleView === "hide") {
            setToggleView("show");
            setPasswordType("text");
        } else {
            setToggleView("hide");
            setPasswordType("password");
        }
    };

    const handleDelete = async () => {
        try {
            await invoke('delete_account', { accountId: id });
            onDelete();
        } catch (error) {
            console.error("Failed to delete user:", error);
        }
    };

    const handleClick = () => {
        if (clickTimeout.current) {
            clearTimeout(clickTimeout.current);
            clickTimeout.current = null;
        } else {
            clickTimeout.current = setTimeout(() => {
                notify();
                clickTimeout.current = null;
            }, 500);
        }
    };

    const addNewPassword = async () => {
        try {
            if (passwordValue.trim() !== "") {
                const website = websiteName.trim() === "" ? 'blank' : websiteName;
                await invoke('insert_account_password', { userId: id, website: website, password: passwordValue });
                handlePasswordClose();

                if (clickTimeout.current) {
                    clearTimeout(clickTimeout.current);
                    clickTimeout.current = null;
                } else {
                    clickTimeout.current = setTimeout(() => {
                        setPasswordListKey(!passwordListKey);
                        clickTimeout.current = null;
                    }, 100);
                }
            }
        } catch (error) {
            console.error("Failed to add password:", error);
        }
    };

    return(
        <div>
            <div className="account-container">
                <p className='account-title'>{emailUsername}</p>
                <div className='action-container'>
                    <button className='dropdown-delete' onDoubleClick={handleDelete} onClick={handleClick}><img className='dropdown-delete-image' src={deleteIcon} alt='delete'/></button>
                    <button className={dropdown ? 'account-liftup' : 'account-dropdown'} onClick={toggleDropdown}><img className='dropdown-image' src={dropdown ? upArrow : downArrow} alt='dropdown'/></button>
                </div>
            </div>
            {dropdown && (
                <div className='password-container'>
                    <PasswordList listKey={passwordListKey} accountID={id}  />
                    {passwordToggle && (
                        <div className='new-password-container'>
                            <div className='new-password-input-container'>
                                <input className='new-password-website' placeholder='Account' value={websiteName} onChange={(e) => setWebsiteName(e.target.value)} />
                                <input className='new-password-input' type={passwordType} placeholder='Password' value={passwordValue} onChange={(e) => setPasswordValue(e.target.value)} />
                            </div>
                            <div className='new-password-btn-container'>
                                <button className='new-password-confirm' onClick={addNewPassword}><img className='new-password-confirm-img' src={confirmIcon} alt='confirm'/></button>
                                <button className='new-password-close' onClick={handlePasswordClose}><img className='new-password-close-img' src={closeIcon} alt='close'/></button>
                                <button className='new-password-toggle-view' onClick={handleToggleView}><img className='new-password-toggle-view-img' src={toggleView === "hide" ? eyeHide : eyeShow} alt="toggle view" /></button>
                            </div>
                        </div>
                    )}
                    <button className="add-password-btn" onClick={() => setPasswordToggle(true)}><img className="add-password-btn-img" src={addIcon} alt="add"/></button>
                </div>
            )}
            <ToastContainer
                position='bottom-center'
                autoClose={1000}
                hideProgressBar={true}
                closeOnClick
                rtl={false}
                theme="dark"
                transition={Bounce}
                closeButton={false}
                limit={1}
            />
        </div>
    );
}

export default Account;
