import React from 'react';

const core: any = window.require('core');

export default () => {
    return (
        <div>{core.hello()}</div>
    );
};
