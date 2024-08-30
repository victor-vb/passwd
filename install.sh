# 如果开机任务失败运行此bash
# mac 系统 注意路径存放，以及所有文件给可读可写权限
# 将 passwd.plist 存放到 /Library/LaunchAgents 
# chown root:wheel passwd.plist
# chmod 644 passwd.plist
# launchctl load /Library/LaunchAgents/passwd.plist
# sudo launchctl start /Library/LaunchAgents/passwd.plist
# ps aux |grep passwd
sudo echo "正在执行脚本"
# 密码文件存放路径
passwd_file_path=
# 运行日志路径
run_log_path=

plist_file_name=wdbox.plist

function install(){
    # 编译程序
    cargo build -r
    #生成plist MAC服务
    cargo run -r -- -f $passwd_file_path service -l $run_log_path

    #全局可执行文件:wdbox
    cargo install --path wdbox

    sudo cp ./$plist_file_name /Library/LaunchAgents

    cd /Library/LaunchAgents
    
    stop

    chown root:wheel $plist_file_name
    chmod 644 $plist_file_name
    launchctl load /Library/LaunchAgents/$plist_file_name
    launchctl start /Library/LaunchAgents/$plist_file_name
    ps aux |grep $passwd_file_path
}


function stop(){
    result=$(ps aux |grep $passwd_file_path)
    if [ ["$result" != ""] ]
    then
        launchctl stop $plist_file_name
        launchctl unload $plist_file_name
    fi
}

install

sudo echo "done!"