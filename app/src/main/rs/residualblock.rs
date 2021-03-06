/*
* Copyright (C) 2016 The Android Open Source Project
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

#pragma version(1)
#pragma rs java_package_name(com.example.android.neural_style)
#pragma rs_fp_relaxed

rs_allocation img_alloc;

// Add the original input and residual output together.
float RS_KERNEL add(float in, uint32_t x, uint32_t y) {
     float img = rsGetElementAt_float(img_alloc, x, y);
     return in + img;
}