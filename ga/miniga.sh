echo "Running GA for atomgit c dataset"
python mini_ga.py ../dataset/atomgit/c/double_free.c ../dataset/atomgit/testc/double_free/ double_free_mini double_free_mini_success/ 
python mini_ga.py ../dataset/atomgit/c/message_queue.c ../dataset/atomgit/testc/message_queue/ message_queue_mini message_queue_mini_success/ 
python mini_ga.py ../dataset/atomgit/c/multi_thread_basic.c ../dataset/atomgit/testc/multi_thread_basic/ multi_thread_basic_mini multi_thread_basic_mini_success/
python mini_ga.py ../dataset/atomgit/c/mutex.c ../dataset/atomgit/testc/mutex/ mutex_mini mutex_mini_success/
python mini_ga.py ../dataset/atomgit/c/queue.c ../dataset/atomgit/testc/queue/ queue_mini queue_mini_success/
python mini_ga.py ../dataset/atomgit/c/queue_l.c ../dataset/atomgit/testc/queue_l/ queue_l_mini queue_l_mini_success/
python mini_ga.py ../dataset/atomgit/c/server_clients.c ../dataset/atomgit/testc/server_clients/ server_clients_mini server_clients_mini_success/
python mini_ga.py ../dataset/atomgit/c/tree.c ../dataset/atomgit/testc/tree/ tree_mini tree_mini_success/
python mini_ga.py ../dataset/atomgit/c/tree_l.c ../dataset/atomgit/testc/tree_l/ tree_l_mini tree_l_mini_success/
echo "Running GA for atomgit cpp dataset"
python mini_ga.py ../dataset/atomgit/cpp/weakptr.cpp ../dataset/atomgit/testcpp/weakptr/ weakptr_mini weakptr_mini_success/
python mini_ga.py ../dataset/atomgit/cpp/weakptr_l.cpp ../dataset/atomgit/testcpp/weakptr_l/ weakptr_l_mini weakptr_l_mini_success/
python mini_ga.py ../dataset/atomgit/cpp/derived.cpp ../dataset/atomgit/testcpp/derived/ derived_mini derived_mini_success/
python mini_ga.py ../dataset/atomgit/cpp/macro_func.cpp ../dataset/atomgit/testcpp/macro_func/ macro_func_mini macro_func_mini_success/
python mini_ga.py ../dataset/atomgit/cpp/var_func.cpp ../dataset/atomgit/testcpp/var_func/ var_func_mini var_func_mini_success/
python mini_ga.py ../dataset/atomgit/cpp/multi_thread_basic.cpp ../dataset/atomgit/testcpp/multi_thread_basic/ multi_thread_basic_cpp_mini multi_thread_basic_cpp_mini_success/
echo "Running GA for codeforce c dataset"
python mini_ga.py ../dataset/codeforce/c/1968A.c ../dataset/codeforce/testc/1968A/ 1968A_mini 1968A_mini_success/
python mini_ga.py ../dataset/codeforce/c/1968B.c ../dataset/codeforce/testc/1968B/ 1968B_mini 1968B_mini_success/
python mini_ga.py ../dataset/codeforce/c/1968C.c ../dataset/codeforce/testc/1968C/ 1968C_mini 1968C_mini_success/
python mini_ga.py ../dataset/codeforce/c/1968D.c ../dataset/codeforce/testc/1968D/ 1968D_mini 1968D_mini_success/
python mini_ga.py ../dataset/codeforce/c/1968E.c ../dataset/codeforce/testc/1968E/ 1968E_mini 1968E_mini_success/
python mini_ga.py ../dataset/codeforce/c/1968F.c ../dataset/codeforce/testc/1968F/ 1968F_mini 1968F_mini_success/
python mini_ga.py ../dataset/codeforce/c/1968G1.c ../dataset/codeforce/testc/1968G1/ 1968G1_mini 1968G1_mini_success/
python mini_ga.py ../dataset/codeforce/c/1968G2.c ../dataset/codeforce/testc/1968G2/ 1968G2_mini 1968G2_mini_success/
echo "Running GA for codeforce cpp dataset"
python mini_ga.py ../dataset/codeforce/cpp/1968A.cpp ../dataset/codeforce/testcpp/1968A/ 1968A_cpp_mini 1968A_cpp_mini_success/
python mini_ga.py ../dataset/codeforce/cpp/1968B.cpp ../dataset/codeforce/testcpp/1968B/ 1968B_cpp_mini 1968B_cpp_mini_success/
python mini_ga.py ../dataset/codeforce/cpp/1968C.cpp ../dataset/codeforce/testcpp/1968C/ 1968C_cpp_mini 1968C_cpp_mini_success/
python mini_ga.py ../dataset/codeforce/cpp/1968D.cpp ../dataset/codeforce/testcpp/1968D/ 1968D_cpp_mini 1968D_cpp_mini_success/
python mini_ga.py ../dataset/codeforce/cpp/1968E.cpp ../dataset/codeforce/testcpp/1968E/ 1968E_cpp_mini 1968E_cpp_mini_success/
python mini_ga.py ../dataset/codeforce/cpp/1968F.cpp ../dataset/codeforce/testcpp/1968F/ 1968F_cpp_mini 1968F_cpp_mini_success/
python mini_ga.py ../dataset/codeforce/cpp/1968G1.cpp ../dataset/codeforce/testcpp/1968G1/ 1968G1_cpp_mini 1968G1_cpp_mini_success/
python mini_ga.py ../dataset/codeforce/cpp/1968G2.cpp ../dataset/codeforce/testcpp/1968G2/ 1968G2_cpp_mini 1968G2_cpp_mini_success/


echo "mv"
mv double_free_mini_success 4omini/success
mv double_free_mini 4omini/result
mv message_queue_mini_success 4omini/success
mv message_queue_mini 4omini/result
mv multi_thread_basic_mini_success 4omini/success
mv multi_thread_basic_mini 4omini/result
mv mutex_mini_success 4omini/success
mv mutex_mini 4omini/result
mv queue_mini_success 4omini/success
mv queue_mini 4omini/result
mv queue_l_mini_success 4omini/success
mv queue_l_mini 4omini/result
mv server_clients_mini_success 4omini/success
mv server_clients_mini 4omini/result
mv tree_mini_success 4omini/success
mv tree_mini 4omini/result
mv tree_l_mini_success 4omini/success
mv tree_l_mini 4omini/result
mv weakptr_mini_success 4omini/success
mv weakptr_mini 4omini/result
mv weakptr_l_mini_success 4omini/success
mv weakptr_l_mini 4omini/result
mv derived_mini_success 4omini/success
mv derived_mini 4omini/result
mv macro_func_mini_success 4omini/success
mv macro_func_mini 4omini/result
mv var_func_mini_success 4omini/success
mv var_func_mini 4omini/result
mv multi_thread_basic_cpp_mini_success 4omini/success
mv multi_thread_basic_cpp_mini 4omini/result
mv 1968A_mini_success 4omini/success
mv 1968A_mini 4omini/result
mv 1968B_mini_success 4omini/success
mv 1968B_mini 4omini/result
mv 1968C_mini_success 4omini/success
mv 1968C_mini 4omini/result
mv 1968D_mini_success 4omini/success
mv 1968D_mini 4omini/result
mv 1968E_mini_success 4omini/success
mv 1968E_mini 4omini/result
mv 1968F_mini_success 4omini/success
mv 1968F_mini 4omini/result
mv 1968G1_mini_success 4omini/success
mv 1968G1_mini 4omini/result
mv 1968G2_mini_success 4omini/success
mv 1968G2_mini 4omini/result
mv 1968A_cpp_mini_success 4omini/success
mv 1968A_cpp_mini 4omini/result
mv 1968B_cpp_mini_success 4omini/success
mv 1968B_cpp_mini 4omini/result
mv 1968C_cpp_mini_success 4omini/success
mv 1968C_cpp_mini 4omini/result
mv 1968D_cpp_mini_success 4omini/success
mv 1968D_cpp_mini 4omini/result
mv 1968E_cpp_mini_success 4omini/success
mv 1968E_cpp_mini 4omini/result
mv 1968F_cpp_mini_success 4omini/success
mv 1968F_cpp_mini 4omini/result
mv 1968G1_cpp_mini_success 4omini/success
mv 1968G1_cpp_mini 4omini/result
mv 1968G2_cpp_mini_success 4omini/success
mv 1968G2_cpp_mini 4omini/result
echo "Done"
