import os
import re
import subprocess
import shutil
import concurrent.futures
import sys
import time
import copy
import random
from requests.exceptions import RequestException, Timeout
from time import sleep
from openai import OpenAI


key = ""
with open("../api/api_key_gpt", "r") as f:
    key = f.read().splitlines()[0]

client = OpenAI(
        api_key = key
        )

temperature_default = float(1.0)
top_default = float(1.0)

def normalize_text(text):
    return "\n".join(line.strip() for line in text.split("\n") if line.strip())


def read_file_content(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        return file.read()

def write_to_file(file_path, content):
    with open(file_path, 'w', encoding='utf-8') as file:
        file.write(content)

def process_file(input_content,temperature_value,top_p_value):
    """Send content to GPT and return the response using the updated API syntax."""
    content = input_content
    content += "Convert/Translate the above code to correct Rust code."
    max_retries = 6
    attempt = 0
    temperature_value = round(temperature_value, 2)
    top_p_value = round(top_p_value, 2)
    while attempt < max_retries:
        try:
            response = client.chat.completions.create(
                model="gpt-3.5-turbo-0125",
                messages=[
                    {"role": "user", "content": content}
                ],
                max_tokens=4096,
                temperature=temperature_value,
                top_p=top_p_value,
                timeout=60
            )
            output=response.choices[0].message.content
            if "```rust" in output:
                rust_codes = re.findall(r'```rust\n?(.*?)\n?```', output, re.DOTALL)
                rust_code = '\n'.join(rust_codes)
            else:
                rust_code = output
            # rust_codes = re.findall(r'```rust\n?(.*?)\n?```', output, re.DOTALL)
            return rust_code
        except Timeout:
            print(f"Request timed out. Retrying... ({attempt + 1}/{max_retries})")
            attempt += 1
            time.sleep(2)  # 在重试之前等待几秒钟
        
        except RequestException as e:
            print(f"An error occurred: {e}")
            return None

    print("Exceeded maximum number of retries. Please try again later.")
    return None

def process_file_double(input_content,temperature_value,top_p_value):
    content = input_content
    content += "First, analyze the following code. Second, provide the following information in a concise manner: 1. I/O: Describe the code's inputs and outputs. 2. Functionality: Summarize the main functionality and purpose of the code. "
    max_retries = 6
    # temperature_value = format(temperature_value, ".2f")
    # top_p_value = format(top_p_value, ".2f")
    temperature_value = round(temperature_value, 2)
    top_p_value = round(top_p_value, 2)
    attempt = 0
    while attempt < max_retries:
        try:
            response_first_round = client.chat.completions.create(
                model="gpt-3.5-turbo-0125",
                messages=[{"role": "user", "content": content}],
            )
            first_round_output = response_first_round.choices[0].message.content
            response = client.chat.completions.create(
                model="gpt-3.5-turbo-0125",
                messages=[
                    {"role": "user", "content": content},
                    {"role": "assistant", "content": first_round_output},
                    {"role": "user", "content": "Provide a complete, compilable Rust code implementation/translation for it."}
                ],
                max_tokens=4096,
                temperature=temperature_value,
                top_p=top_p_value,
                timeout=60
            )
            output=response.choices[0].message.content
            if "```rust" in output:
                rust_codes = re.findall(r'```rust\n?(.*?)\n?```', output, re.DOTALL)
                rust_code = '\n'.join(rust_codes)
            else:
                rust_code = output
            return rust_code 
        except Timeout:
            print(f"Request timed out. Retrying... ({attempt + 1}/{max_retries})")
            attempt += 1
            time.sleep(2)  
        
        except RequestException as e:
            print(f"An error occurred: {e}")
            return None

class ModelGenome:
    def __init__(self, double_prompt: int, temperature: float, top_p: float, temperature_direction: int, top_p_direction: int, fixed_parameter: int, weight: int):
        self.double_prompt = double_prompt
        self.temperature = temperature
        self.top_p = top_p
        self.temperature_direction = temperature_direction
        self.top_p_direction = top_p_direction
        self.fixed_parameter = fixed_parameter
        self.weight = weight


def copy_rust_code_to_main_rs(rust_code):
    # Copy rust code to rust_tests/src/main.rs
    with open('rust_tests/src/main.rs', 'w') as f:
        f.write(rust_code)

def cargo_build():
    # Run cargo build
    try:
        os.chdir('rust_tests')
        result = subprocess.run(['cargo', 'build'], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        os.chdir('..')
        if result.returncode != 0:
            print(result.stderr.decode())
            return False
        return True
    except Exception as e:
        print(f"Build failed: {e}")
        os.chdir('..')
        return False

def cargo_run(output):
    print("cargo_run")
    # Run cargo run without inputs
    try:
        os.chdir('rust_tests')
        result = subprocess.run('cargo run', shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10)
        os.chdir('..')
        if result.returncode != 0:
            print(result.stderr.decode())
            return False
        result_content = result.stdout.decode()
        output_s = normalize_text(output)
        result_s = normalize_text(result_content)
        return output_s == result_s
        # print(result_content)
        # return result_content.strip() == output.strip()
    except subprocess.TimeoutExpired:
        print("Timeout")
        os.chdir('..')
        return False
    except Exception as e:
        print(f"Run failed: {e}")
        os.chdir('..')
        return False

def input_cargo_run(input_data, output):
    # Run cargo run with inputs
    try:
        os.chdir('rust_tests')
        with open('input.txt', 'w') as f:
            f.write(input_data)
        result = subprocess.run('cargo run <input.txt', shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=60)
        os.remove('input.txt')
        os.chdir('..')
        if result.returncode != 0:
            print(result.stderr.decode())
            return False
        result_content = result.stdout.decode()
        output_s = normalize_text(output)
        result_s = normalize_text(result_content)
        # output_s = "\n".join([line for line in output.split("\n") if line.strip()])
        # result_s = "\n".join([line for line in result_content.split("\n") if line.strip()])
        # return result_content.strip() == output.strip()
        return output_s == result_s
    except subprocess.TimeoutExpired:
        print("Timeout")
        os.remove('input.txt')
        os.chdir('..')
        return False
    except Exception as e:
        print(f"Run failed: {e}")
        os.remove('input.txt')
        os.chdir('..')
        return False

def test_script(rust_code, test_dir):
    try:
        copy_rust_code_to_main_rs(rust_code)

        build_success = cargo_build()
        if not build_success:
            return 0  # Return 0 if the build fails

        test_cases = []
        input_dir = os.path.join(test_dir, 'input')
        output_dir = os.path.join(test_dir, 'output')
        if os.path.exists(input_dir):
            for input_file in os.listdir(input_dir):
                with open(os.path.join(input_dir, input_file), 'r') as f:
                    input_data = f.read()
                with open(os.path.join(output_dir, input_file), 'r') as f:
                    output_data = f.read()
                test_cases.append({'input': input_data, 'output': output_data})
        else:
            for output_file in os.listdir(output_dir):
                with open(os.path.join(output_dir, output_file), 'r') as f:
                    output_data = f.read()
                test_cases.append({'output': output_data})

        for test_case in test_cases:
            input_data = test_case.get('input', None)
            output_data = test_case['output']
            if input_data:
                if not input_cargo_run(input_data, output_data):
                    return 1
            else:
                if not cargo_run(output_data):
                    return 1
        return 2
        
    except Exception as e:
        print(f"Exception during testing: {e}")
        return 0  # Return 0 if any exception occurs


def run_individual(genome: ModelGenome,input_content):
    # print(input_content)
    if genome.double_prompt == 0:
        if genome.fixed_parameter == 0:
            rust_code = process_file(input_content, temperature_default, genome.top_p)
        else:
            rust_code = process_file(input_content, genome.temperature, top_default)
    else:
        if genome.fixed_parameter == 0:
            rust_code = process_file_double(input_content, temperature_default, genome.top_p)
        else:
            rust_code = process_file_double(input_content, genome.temperature, top_default)
    return rust_code


def mutex(individuals: list):
    # for each individual, percent 0.05 to mutex A, D, E, F
    for individual in individuals:
        random_value = random.uniform(0, 1)
        if random_value < 0.05:
            individual.weight = 1
            individual.double_prompt = 1 - individual.double_prompt
            individual.fixed_parameter = 1 - individual.fixed_parameter
            individual.temperature_direction = 1 - individual.temperature_direction
            individual.top_p_direction = 1 - individual.top_p_direction
    return individuals



def crossover(individuals: list):
    numbers1 = random.sample([0, 1, 2, 3], 2)
    remaining_indices = [i for i in [0, 1, 2, 3] if i not in numbers1]
    numbers2 = remaining_indices
    # numbers1 = [0,1]
    # numbers2 = [2,3]
    
    attributes = ['double_prompt', 'fixed_parameter', 'temperature_direction', 'top_p_direction']
    
    for attr in attributes:
        random_value = random.uniform(0, 1)
        if random_value < 0.6:
            index1 = numbers1[0]
            individuals[index1].weight = 1
            index2 = numbers1[1]
            individuals[index2].weight = 1
            temp = getattr(individuals[index1], attr)
            setattr(individuals[index1], attr, getattr(individuals[index2], attr))
            setattr(individuals[index2], attr, temp)

    for attr in attributes:
        random_value = random.uniform(0, 1)
        if random_value < 0.6:
            index1 = numbers2[0]
            individuals[index1].weight = 1
            index2 = numbers2[1]
            individuals[index2].weight = 1
            temp = getattr(individuals[index1], attr)
            setattr(individuals[index1], attr, getattr(individuals[index2], attr))
            setattr(individuals[index2], attr, temp)
    return individuals


def local_search(individuals: list):
    for individual in individuals:
        individual.weight = 1
        if individual.temperature_direction == 0:
            individual.temperature = individual.temperature - 0.1
        else:
            individual.temperature = individual.temperature + 0.1
        if individual.temperature >= 2.0:
            individual.temperature = 2.0
        if individual.temperature <= 0.0:
            individual.temperature = 0.0
        if individual.top_p_direction == 0:
            individual.top_p = individual.top_p - 0.1
        else:
            individual.top_p = individual.top_p + 0.1
        if individual.top_p >= 1:
            individual.top_p = 1
        if individual.top_p <= 0.0:
            individual.top_p = 0.0
    return individuals


def GA_gen(input_content, test_dir, individuals: list):
    result_codes = []
    result_values = []
    for individual in individuals:
        codes = run_individual(individual, input_content)
        # print(codes)
        result_codes.append(codes)
    # print(result_codes)
    for i in range(len(result_codes)):
        result_code = result_codes[i]
        # print(result_code)
        result_values.append(test_script(result_code, test_dir))
    return result_codes, result_values


def roulette_wheel_selection(individuals:list, num_to_select):
    individuals_copy = copy.deepcopy(individuals)
    selected = []
    
    for _ in range(num_to_select):
        total_weight = sum(ind.weight for ind in individuals_copy)
        pick = random.uniform(0, total_weight)
        current = 0
        
        for ind in individuals_copy:
            current += ind.weight
            if current > pick:
                selected.append(ind)
                individuals_copy.remove(ind)
                break
    
    return selected


if __name__ == '__main__':
    input_file = sys.argv[1]
    test_dir = sys.argv[2]
    result_dir = sys.argv[3]
    success_dir = sys.argv[4]
    input_content = read_file_content(input_file)
    # if not exit result_dir, create it
    if not os.path.exists(result_dir):
        os.makedirs(result_dir)
    # if not exit success_dir, create it
    if not os.path.exists(success_dir):
        os.makedirs(success_dir)
    glm_individ_1 = ModelGenome(
        double_prompt = 0,
        temperature = 1,
        top_p = 1,
        temperature_direction = 0,
        top_p_direction = 0,
        fixed_parameter = 0,
        weight = 1
    )
    glm_individ_2 = ModelGenome(
        double_prompt = 0,
        temperature = 1,
        top_p = 1,
        temperature_direction = 0,
        top_p_direction = 0,
        fixed_parameter = 1,
        weight = 1
    )
    glm_individ_3 = ModelGenome(
        double_prompt = 0,
        temperature = 1,
        top_p = 1,
        temperature_direction = 1,
        top_p_direction = 1,
        fixed_parameter = 0,
        weight = 1
    )
    glm_individ_4 = ModelGenome(
        double_prompt = 0,
        temperature = 1,
        top_p = 1,
        temperature_direction = 1,
        top_p_direction = 1,
        fixed_parameter = 1,
        weight = 1
    )
    glm_individ_5 = ModelGenome(
        double_prompt = 0,
        temperature = 1,
        top_p = 1,
        temperature_direction = 1,
        top_p_direction = 1,
        fixed_parameter = 1,
        weight = 1
    )
    rust_codes = []
    result_values = []
    # The initial value of double_prompt is 1 with a probability value of 0.5
    random_value = random.uniform(0, 1)
    if random_value < 0.5:
        glm_individ_1.double_prompt = 1
    random_value = random.uniform(0, 1)
    if random_value < 0.5:
        glm_individ_2.double_prompt = 1
    random_value = random.uniform(0, 1)
    if random_value < 0.5:
        glm_individ_3.double_prompt = 1
    random_value = random.uniform(0, 1)
    if random_value < 0.5:
        glm_individ_4.double_prompt = 1
    random_value = random.uniform(0, 1)
    if random_value < 0.5:
        glm_individ_5.double_prompt = 1
        
    init_individuals = [glm_individ_1, glm_individ_2, glm_individ_3, glm_individ_4, glm_individ_5]
    # print(init_individuals)
    # first generation
    rust_codes, result_values = GA_gen(input_content, test_dir, init_individuals)
    results_rusts = []
    success_rusts = []
    for i in range(len(result_values)):
        # print(rust_codes[i])
        results_rusts.append(rust_codes[i])
        if result_values[i] == 1:
            init_individuals[i].weight = init_individuals[i].weight + 1
        if result_values[i] == 2:
            init_individuals[i].weight = init_individuals[i].weight + 10
            success_rusts.append(rust_codes[i])
            with open(success_dir + f"iters", "a") as f:
                f.write(str(0)+str(i)+"\n")

    individuals_list = []
    # individuals_local = []
    for individual in init_individuals:
        individuals_list.append(individual)
        # individuals_local.append(individual)
    # select next generation
    for i in range(9):
        individuals = roulette_wheel_selection(individuals_list, 5)
        # individuals_local = []
        individuals = mutex(individuals)
        individuals = crossover(individuals)
        individuals = local_search(individuals)
        # sleep(2000)
        rust_codes, result_values = GA_gen(input_content, test_dir, individuals)
        for j in range(len(result_values)):
            results_rusts.append(rust_codes[j])
            if result_values[j] == 0:
                with open(success_dir + f"sums", "a") as f:
                    f.write(str(i)+str(j)+"--\n")
            if result_values[j] == 1:
                individuals[j].weight = individuals[j].weight + 1
                with open(success_dir + f"sums", "a") as f:
                    f.write(str(i)+str(j)+"-\n")
            if result_values[j] == 2:
                individuals[j].weight = individuals[j].weight + 10
                with open(success_dir + f"sums", "a") as f:
                    f.write(str(i)+str(j)+"+\n")
                with open(success_dir + f"iters", "a") as f:
                    f.write(str(i)+str(j)+"\n")
            if individuals[j].weight > 100:
                individuals[j].weight = 100
                # success_rusts.append(rust_codes[j])
        for individual in individuals:
            individuals_list.append(individual)
    # write to file in result_dir + file_name
    for i in range(len(results_rusts)):
        write_to_file(result_dir + f"/{str(i)}_{os.path.basename(input_file)}", results_rusts[i])
    # write to file in result_dir + success_rusts
    # for i in range(len(success_rusts)):
    #     write_to_file(success_dir + f"/{str(i)}_{os.path.basename(input_file)}", success_rusts[i])
    # write to file: the first success_rusts in the location of result_rusts
