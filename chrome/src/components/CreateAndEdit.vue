<template id="create_and_edit">
    <form action="">
        <div class="row">
            <div class="col-12">
                <label for="account_id">
                    账户信息:
                </label>
                <input type="text" name="account" id="account_id" class="form-control" placeholder="请输入您的账户">
            </div>
        </div>
        <div class="row">
            <div class="col-12">
                <label for="password_id">
                    密码信息:
                </label>
                <input type="password" name="password" id="password_id" class="form-control" placeholder="请输入您的密码">
            </div>
        </div>

        <div class="row">
            <div class="col-12">
                <label for="google_2fa_id">
                    Google 2FA:
                </label>
                <input type="password" name="google_2fa" id="google_2fa_id" class="form-control"
                    placeholder="请输入Google 密钥串，没有可以不填写">
            </div>
        </div>
        <div class="row">
            <div class="col-12">
                <label for="urls_id">
                    输入网址:
                </label>
                <textarea name="urls" id="urls_id" class="form-control" rows="6"
                    placeholder="请输入网址:http(s)?://www.baidu.com/&#10;一行一个网址，以#开头的的作为备注选择"></textarea>
            </div>
        </div>
        <div class="row">
            <div class="col-12">
                <label for="remark_id">
                    账户备注:
                </label>
                <textarea name="remark" id="remark_id" class="form-control" rows="4" placeholder="输入备注信息"></textarea>
            </div>
        </div>
        <div class="row">
            <div class="col-6 offset-6 mt-2">
                <input type="button" value="抓取host" class="btn btn-primary btn-sm me-2 ms-4">
                <input type="submit" value="提交" class="btn btn-primary btn-sm">
            </div>
        </div>
    </form>
</template>
