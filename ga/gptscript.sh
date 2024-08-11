echo "Running GA for atomgit c dataset"
python gpt_ga.py ../dataset/atomgit/c/double_free.c ../dataset/atomgit/testc/double_free/ double_free_gpt double_free_gpt_success/ 
python gpt_ga.py ../dataset/atomgit/c/message_queue.c ../dataset/atomgit/testc/message_queue/ message_queue_gpt message_queue_gpt_success/ 
python gpt_ga.py ../dataset/atomgit/c/multi_thread_basic.c ../dataset/atomgit/testc/multi_thread_basic/ multi_thread_basic_gpt multi_thread_basic_gpt_success/
python gpt_ga.py ../dataset/atomgit/c/mutex.c ../dataset/atomgit/testc/mutex/ mutex_gpt mutex_gpt_success/
python gpt_ga.py ../dataset/atomgit/c/queue.c ../dataset/atomgit/testc/queue/ queue_gpt queue_gpt_success/
python gpt_ga.py ../dataset/atomgit/c/queue_l.c ../dataset/atomgit/testc/queue_l/ queue_l_gpt queue_l_gpt_success/
python gpt_ga.py ../dataset/atomgit/c/server_clients.c ../dataset/atomgit/testc/server_clients/ server_clients_gpt server_clients_gpt_success/
python gpt_ga.py ../dataset/atomgit/c/tree.c ../dataset/atomgit/testc/tree/ tree_gpt tree_gpt_success/
python gpt_ga.py ../dataset/atomgit/c/tree_l.c ../dataset/atomgit/testc/tree_l/ tree_l_gpt tree_l_gpt_success/
echo "Running GA for atomgit cpp dataset"
python gpt_ga.py ../dataset/atomgit/cpp/weakptr.cpp ../dataset/atomgit/testcpp/weakptr/ weakptr_gpt weakptr_gpt_success/
python gpt_ga.py ../dataset/atomgit/cpp/weakptr_l.cpp ../dataset/atomgit/testcpp/weakptr_l/ weakptr_l_gpt weakptr_l_gpt_success/
python gpt_ga.py ../dataset/atomgit/cpp/derived.cpp ../dataset/atomgit/testcpp/derived/ derived_gpt derived_gpt_success/
python gpt_ga.py ../dataset/atomgit/cpp/macro_func.cpp ../dataset/atomgit/testcpp/macro_func/ macro_func_gpt macro_func_gpt_success/
python gpt_ga.py ../dataset/atomgit/cpp/var_func.cpp ../dataset/atomgit/testcpp/var_func/ var_func_gpt var_func_gpt_success/
python gpt_ga.py ../dataset/atomgit/cpp/multi_thread_basic.cpp ../dataset/atomgit/testcpp/multi_thread_basic/ multi_thread_basic_cpp_gpt multi_thread_basic_cpp_gpt_success/
echo "Running GA for codeforce c dataset"
python gpt_ga.py ../dataset/codeforce/c/1968A.c ../dataset/codeforce/testc/1968A/ 1968A_gpt 1968A_gpt_success/
python gpt_ga.py ../dataset/codeforce/c/1968B.c ../dataset/codeforce/testc/1968B/ 1968B_gpt 1968B_gpt_success/
python gpt_ga.py ../dataset/codeforce/c/1968C.c ../dataset/codeforce/testc/1968C/ 1968C_gpt 1968C_gpt_success/
python gpt_ga.py ../dataset/codeforce/c/1968D.c ../dataset/codeforce/testc/1968D/ 1968D_gpt 1968D_gpt_success/
python gpt_ga.py ../dataset/codeforce/c/1968E.c ../dataset/codeforce/testc/1968E/ 1968E_gpt 1968E_gpt_success/
python gpt_ga.py ../dataset/codeforce/c/1968F.c ../dataset/codeforce/testc/1968F/ 1968F_gpt 1968F_gpt_success/
python gpt_ga.py ../dataset/codeforce/c/1968G1.c ../dataset/codeforce/testc/1968G1/ 1968G1_gpt 1968G1_gpt_success/
python gpt_ga.py ../dataset/codeforce/c/1968G2.c ../dataset/codeforce/testc/1968G2/ 1968G2_gpt 1968G2_gpt_success/
echo "Running GA for codeforce cpp dataset"
python gpt_ga.py ../dataset/codeforce/cpp/1968A.cpp ../dataset/codeforce/testcpp/1968A/ 1968A_cpp_gpt 1968A_cpp_gpt_success/
python gpt_ga.py ../dataset/codeforce/cpp/1968B.cpp ../dataset/codeforce/testcpp/1968B/ 1968B_cpp_gpt 1968B_cpp_gpt_success/
python gpt_ga.py ../dataset/codeforce/cpp/1968C.cpp ../dataset/codeforce/testcpp/1968C/ 1968C_cpp_gpt 1968C_cpp_gpt_success/
python gpt_ga.py ../dataset/codeforce/cpp/1968D.cpp ../dataset/codeforce/testcpp/1968D/ 1968D_cpp_gpt 1968D_cpp_gpt_success/
python gpt_ga.py ../dataset/codeforce/cpp/1968E.cpp ../dataset/codeforce/testcpp/1968E/ 1968E_cpp_gpt 1968E_cpp_gpt_success/
python gpt_ga.py ../dataset/codeforce/cpp/1968F.cpp ../dataset/codeforce/testcpp/1968F/ 1968F_cpp_gpt 1968F_cpp_gpt_success/
python gpt_ga.py ../dataset/codeforce/cpp/1968G1.cpp ../dataset/codeforce/testcpp/1968G1/ 1968G1_cpp_gpt 1968G1_cpp_gpt_success/
python gpt_ga.py ../dataset/codeforce/cpp/1968G2.cpp ../dataset/codeforce/testcpp/1968G2/ 1968G2_cpp_gpt 1968G2_cpp_gpt_success/


echo "mv"
mv double_free_gpt_success gpt3_5/success
mv double_free_gpt gpt3_5/result
mv message_queue_gpt_success gpt3_5/success
mv message_queue_gpt gpt3_5/result
mv multi_thread_basic_gpt_success gpt3_5/success
mv multi_thread_basic_gpt gpt3_5/result
mv mutex_gpt_success gpt3_5/success
mv mutex_gpt gpt3_5/result
mv queue_gpt_success gpt3_5/success
mv queue_gpt gpt3_5/result
mv queue_l_gpt_success gpt3_5/success
mv queue_l_gpt gpt3_5/result
mv server_clients_gpt_success gpt3_5/success
mv server_clients_gpt gpt3_5/result
mv tree_gpt_success gpt3_5/success
mv tree_gpt gpt3_5/result
mv tree_l_gpt_success gpt3_5/success
mv tree_l_gpt gpt3_5/result
mv weakptr_gpt_success gpt3_5/success
mv weakptr_gpt gpt3_5/result
mv weakptr_l_gpt_success gpt3_5/success
mv weakptr_l_gpt gpt3_5/result
mv derived_gpt_success gpt3_5/success
mv derived_gpt gpt3_5/result
mv macro_func_gpt_success gpt3_5/success
mv macro_func_gpt gpt3_5/result
mv var_func_gpt_success gpt3_5/success
mv var_func_gpt gpt3_5/result
mv multi_thread_basic_cpp_gpt_success gpt3_5/success
mv multi_thread_basic_cpp_gpt gpt3_5/result
mv 1968A_gpt_success gpt3_5/success
mv 1968A_gpt gpt3_5/result
mv 1968B_gpt_success gpt3_5/success
mv 1968B_gpt gpt3_5/result
mv 1968C_gpt_success gpt3_5/success
mv 1968C_gpt gpt3_5/result
mv 1968D_gpt_success gpt3_5/success
mv 1968D_gpt gpt3_5/result
mv 1968E_gpt_success gpt3_5/success
mv 1968E_gpt gpt3_5/result
mv 1968F_gpt_success gpt3_5/success
mv 1968F_gpt gpt3_5/result
mv 1968G1_gpt_success gpt3_5/success
mv 1968G1_gpt gpt3_5/result
mv 1968G2_gpt_success gpt3_5/success
mv 1968G2_gpt gpt3_5/result
mv 1968A_cpp_gpt_success gpt3_5/success
mv 1968A_cpp_gpt gpt3_5/result
mv 1968B_cpp_gpt_success gpt3_5/success
mv 1968B_cpp_gpt gpt3_5/result
mv 1968C_cpp_gpt_success gpt3_5/success
mv 1968C_cpp_gpt gpt3_5/result
mv 1968D_cpp_gpt_success gpt3_5/success
mv 1968D_cpp_gpt gpt3_5/result
mv 1968E_cpp_gpt_success gpt3_5/success
mv 1968E_cpp_gpt gpt3_5/result
mv 1968F_cpp_gpt_success gpt3_5/success
mv 1968F_cpp_gpt gpt3_5/result
mv 1968G1_cpp_gpt_success gpt3_5/success
mv 1968G1_cpp_gpt gpt3_5/result
mv 1968G2_cpp_gpt_success gpt3_5/success
mv 1968G2_cpp_gpt gpt3_5/result
echo "Done"

echo "rename"
python rename.py gpt3_5
echo "Done"

